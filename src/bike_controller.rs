use kdri::{KettlerConnection, scan_devices};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::collections::HashMap;
use tokio::time;
use anyhow::{Result, bail};
use crate::training_program::{TrainingProgram, ProgramExecutionState};

#[derive(Debug, Clone)]
pub struct BikeData {
    pub speed: f32,
    pub rpm: u16,
    pub power: u16,
    pub connected: bool,
}

pub struct BikeController {
    connection: Arc<Mutex<Option<KettlerConnection>>>,
    data: Arc<Mutex<BikeData>>,
    reconnect_attempts: Arc<Mutex<u32>>,
    // Stockage des programmes d'entraînement
    programs: Arc<Mutex<HashMap<String, TrainingProgram>>>,
    // État du programme en cours d'exécution
    active_program: Arc<Mutex<Option<ProgramExecutionState>>>,
}

impl BikeController {
    // Initialise le contrôleur sans nécessairement se connecter immédiatement
    pub async fn new() -> Result<Arc<Self>> {
        let data = BikeData {
            speed: 0.0,
            rpm: 0,
            power: 0,
            connected: false,
        };

        let controller = Arc::new(BikeController {
            connection: Arc::new(Mutex::new(None)),
            data: Arc::new(Mutex::new(data)),
            reconnect_attempts: Arc::new(Mutex::new(0)),
            programs: Arc::new(Mutex::new(HashMap::new())),
            active_program: Arc::new(Mutex::new(None)),
        });

        // Lancer la tentative de connexion en arrière-plan
        let controller_clone = controller.clone();
        tokio::spawn(async move {
            controller_clone.try_initial_connection().await;
        });

        Ok(controller)
    }

    async fn try_initial_connection(&self) {
        println!("🔍 Recherche d'appareils Kettler...");
        println!("💡 Assurez-vous que le vélo est allumé et en mode Bluetooth");

        // Tentatives initiales avec backoff exponentiel
        let max_attempts = 5;
        for attempt in 1..=max_attempts {
            // Délai avec backoff exponentiel : 2, 4, 8, 16 secondes
            if attempt > 1 {
                let delay = 2u64.pow(attempt - 2);
                println!("⏳ Attente de {} secondes avant nouvelle tentative...", delay);
                tokio::time::sleep(Duration::from_secs(delay)).await;
            }

            match self.attempt_connection().await {
                Ok(_) => {
                    println!("✅ Connecté avec succès après {} tentative(s) !", attempt);
                    return;
                }
                Err(e) => {
                    eprintln!("⚠️  Tentative {}/{} échouée : {:?}", attempt, max_attempts, e);
                }
            }
        }

        println!("⚠️  Impossible de se connecter pour le moment.");
        println!("   Le serveur continue de fonctionner. Réessai automatique toutes les 30 secondes...");
        println!("   Vérifiez que le vélo est allumé et en mode Bluetooth.");

        // Continuer à essayer en arrière-plan avec backoff
        let controller = self;
        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;
            println!("🔄 Tentative de connexion automatique...");
            if let Ok(_) = controller.attempt_connection().await {
                println!("✅ Connecté avec succès !");
                break;
            }
        }
    }

    async fn attempt_connection(&self) -> Result<()> {
        // Nettoyer l'ancienne connexion si elle existe
        {
            let mut conn = self.connection.lock().unwrap();
            if conn.is_some() {
                println!("🧹 Nettoyage de l'ancienne connexion...");
                *conn = None;
                // Laisser le temps au Bluetooth de se libérer
                drop(conn);
                std::thread::sleep(Duration::from_millis(500));
            }
        }

        let connection = Arc::clone(&self.connection);

        let new_conn = tokio::task::spawn_blocking(move || {
            // Essayer plusieurs scans si nécessaire
            let mut devices = Vec::new();
            let max_scan_attempts = 5; // Augmenté de 3 à 5

            for scan_attempt in 1..=max_scan_attempts {
                println!("🔍 Scan Bluetooth {}/{}...", scan_attempt, max_scan_attempts);

                // Délai progressif avant chaque scan (sauf le premier)
                if scan_attempt > 1 {
                    let delay = 3 + (scan_attempt - 1); // 3, 4, 5, 6 secondes
                    println!("   Attente de {} secondes avant le scan...", delay);
                    std::thread::sleep(Duration::from_secs(delay as u64));
                }

                match scan_devices() {
                    Ok(found_devices) => {
                        if !found_devices.is_empty() {
                            devices = found_devices;
                            println!("✓ {} appareil(s) Kettler détecté(s)", devices.len());
                            break;
                        } else {
                            println!("⚠️  Aucun appareil trouvé lors du scan {}/{}", scan_attempt, max_scan_attempts);
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Erreur scan {}/{} : {:?}", scan_attempt, max_scan_attempts, e);
                    }
                }
            }

            if devices.is_empty() {
                return Err(anyhow::anyhow!("Aucun appareil Kettler trouvé après {} scans", max_scan_attempts));
            }

            let device = devices.into_iter().last().ok_or_else(|| anyhow::anyhow!("No Kettler device found"))?;
            println!("📱 Appareil sélectionné : {}", device.get_name());
            println!("   Adresse : {}", device.get_addr().to_string());

            // Délai augmenté pour une meilleure stabilité
            println!("⏳ Attente de 3 secondes pour stabilisation du périphérique...");
            std::thread::sleep(Duration::from_secs(3));

            println!("🔗 Connexion en cours...");

            // Tentative de connexion avec retry
            let mut conn_result = device.connect();
            if conn_result.is_err() {
                println!("⚠️  Première tentative de connexion échouée, réessai dans 2 secondes...");
                std::thread::sleep(Duration::from_secs(2));
                conn_result = device.connect();
            }

            conn_result.map_err(|e| anyhow::anyhow!("Échec connexion après 2 tentatives: {}", e))
        }).await??;

        *connection.lock().unwrap() = Some(new_conn);

        {
            let mut data = self.data.lock().unwrap();
            data.connected = true;
        }

        *self.reconnect_attempts.lock().unwrap() = 0;

        Ok(())
    }

    async fn reconnect(&self) -> Result<()> {
        // Vérifier d'abord si on est déjà connecté
        {
            let conn = self.connection.lock().unwrap();
            if conn.is_some() {
                println!("ℹ️  Déjà connecté, pas besoin de reconnexion");
                return Ok(());
            }
        }

        let current_attempts = {
            let mut attempts = self.reconnect_attempts.lock().unwrap();
            *attempts += 1;
            *attempts
        };

        if current_attempts > 5 {
            println!("❌ Trop de tentatives de reconnexion, réinitialisation du compteur");
            *self.reconnect_attempts.lock().unwrap() = 0;
            return Err(anyhow::anyhow!("Max reconnection attempts reached, will retry later"));
        }

        println!("🔄 Tentative de reconnexion ({}/5)...", current_attempts);

        self.attempt_connection().await?;

        println!("✅ Reconnecté avec succès !");
        Ok(())
    }

    pub fn start_polling(self: Arc<Self>, interval_sec: u64) {
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(interval_sec));
            let mut consecutive_errors = 0u32;

            loop {
                interval.tick().await;
                match self.update_data().await {
                    Ok(_) => {
                        consecutive_errors = 0;
                    }
                    Err(e) => {
                        consecutive_errors += 1;
                        eprintln!("⚠️  Erreur mise à jour données vélo : {:?}", e);

                        if consecutive_errors >= 3 {
                            // Vérifier si on est vraiment déconnecté avant de tenter une reconnexion
                            let is_connected = {
                                let conn = self.connection.lock().unwrap();
                                conn.is_some()
                            };

                            if !is_connected {
                                eprintln!("🔌 Connexion perdue, tentative de reconnexion...");

                                // Marquer comme déconnecté
                                {
                                    let mut data = self.data.lock().unwrap();
                                    data.connected = false;
                                }

                                // Tentative de reconnexion
                                if let Err(reconnect_err) = self.reconnect().await {
                                    eprintln!("❌ Échec de reconnexion : {:?}", reconnect_err);
                                    time::sleep(Duration::from_secs(5)).await;
                                } else {
                                    consecutive_errors = 0;
                                }
                            } else {
                                // Connexion existe mais erreur de communication
                                // Attendre un peu avant de réessayer
                                eprintln!("⚠️  Erreur de communication (connexion existe)");
                                time::sleep(Duration::from_secs(2)).await;
                                consecutive_errors = 0;
                            }
                        }
                    }
                }
            }
        });
    }

    async fn update_data(&self) -> Result<()> {
        let connection = Arc::clone(&self.connection);

        // On doit exécuter toutes les opérations kdri dans un seul spawn_blocking
        let result = tokio::task::spawn_blocking(move || {
            let mut conn_guard = connection.lock().unwrap();

            if let Some(ref mut conn) = *conn_guard {
                let speed = conn.get_speed();
                let rpm = conn.get_rpm();
                let power = conn.get_power_target();
                Ok((speed, rpm, power))
            } else {
                Err(anyhow::anyhow!("No active connection"))
            }
        }).await?;

        let (speed_opt, rpm_opt, power_opt) = result?;

        let speed = speed_opt.map(|v| v as f32 / 10.0).unwrap_or(0.0);
        let rpm = rpm_opt.unwrap_or(0);
        let power = power_opt.unwrap_or(0);

        let mut data = self.data.lock().unwrap();
        data.speed = speed;
        data.rpm = rpm;
        data.power = power;
        data.connected = true;

        Ok(())
    }

    pub async fn get_data(&self) -> BikeData {
        self.data.lock().unwrap().clone()
    }

    pub async fn get_power(&self) -> u16 {
        self.data.lock().unwrap().power
    }

    pub async fn set_power(&self, level: u16) -> Result<()> {
        if level > 400 {
            bail!("Niveau de puissance hors plage (0-400)");
        }

        let connection = Arc::clone(&self.connection);

        tokio::task::spawn_blocking(move || {
            let mut conn_guard = connection.lock().unwrap();
            if let Some(ref mut conn) = *conn_guard {
                conn.set_power(level);
                Ok(())
            } else {
                Err(anyhow::anyhow!("No active connection"))
            }
        }).await??;

        let mut data = self.data.lock().unwrap();
        data.power = level;

        println!("⚡ Puissance définie à {}W", level);

        Ok(())
    }

    // ===== Gestion des programmes d'entraînement =====

    /// Crée un nouveau programme d'entraînement
    pub async fn create_program(&self, program: TrainingProgram) -> Result<()> {
        if !program.is_valid() {
            bail!("Programme invalide : vérifiez que tous les intervalles ont une durée > 0 et une puissance <= 400W");
        }

        let mut programs = self.programs.lock().unwrap();

        if programs.contains_key(&program.id) {
            bail!("Un programme avec l'ID '{}' existe déjà", program.id);
        }

        println!("📝 Nouveau programme créé : {} ({} intervalles, {}s total)",
                 program.name, program.intervals.len(), program.total_duration());

        programs.insert(program.id.clone(), program);
        Ok(())
    }

    /// Met à jour un programme existant
    pub async fn update_program(&self, program: TrainingProgram) -> Result<()> {
        if !program.is_valid() {
            bail!("Programme invalide");
        }

        let mut programs = self.programs.lock().unwrap();

        if !programs.contains_key(&program.id) {
            bail!("Programme '{}' introuvable", program.id);
        }

        // Vérifier qu'on ne modifie pas un programme en cours d'exécution
        let active = self.active_program.lock().unwrap();
        if let Some(ref state) = *active {
            if state.program_id == program.id {
                bail!("Impossible de modifier un programme en cours d'exécution");
            }
        }

        println!("📝 Programme mis à jour : {}", program.name);
        programs.insert(program.id.clone(), program);
        Ok(())
    }

    /// Supprime un programme
    pub async fn delete_program(&self, program_id: &str) -> Result<()> {
        // Vérifier qu'on ne supprime pas un programme en cours d'exécution
        let active = self.active_program.lock().unwrap();
        if let Some(ref state) = *active {
            if state.program_id == program_id {
                bail!("Impossible de supprimer un programme en cours d'exécution");
            }
        }

        let mut programs = self.programs.lock().unwrap();
        if programs.remove(program_id).is_some() {
            println!("🗑️  Programme '{}' supprimé", program_id);
            Ok(())
        } else {
            bail!("Programme '{}' introuvable", program_id);
        }
    }

    /// Liste tous les programmes
    pub async fn list_programs(&self) -> Vec<TrainingProgram> {
        let programs = self.programs.lock().unwrap();
        programs.values().cloned().collect()
    }

    /// Obtient un programme par son ID
    pub async fn get_program(&self, program_id: &str) -> Option<TrainingProgram> {
        let programs = self.programs.lock().unwrap();
        programs.get(program_id).cloned()
    }

    /// Démarre l'exécution d'un programme
    pub async fn start_program(&self, program_id: &str) -> Result<()> {
        // Vérifier qu'aucun programme n'est en cours
        {
            let active = self.active_program.lock().unwrap();
            if active.is_some() {
                bail!("Un programme est déjà en cours d'exécution. Arrêtez-le d'abord.");
            }
        }

        // Récupérer le programme
        let program = {
            let programs = self.programs.lock().unwrap();
            programs.get(program_id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Programme '{}' introuvable", program_id))?
        };

        if !program.is_valid() {
            bail!("Programme invalide");
        }

        // Créer l'état d'exécution
        let state = ProgramExecutionState::new(program);

        // Définir la puissance initiale
        self.set_power(state.current_power_target).await?;

        println!("🎯 Démarrage du programme : {}", state.program_name);
        println!("   Durée totale : {}s", state.total_duration);
        println!("   Premier intervalle : {}W", state.current_power_target);

        *self.active_program.lock().unwrap() = Some(state);

        Ok(())
    }

    /// Démarre la boucle de mise à jour du programme (à appeler après start_program)
    pub fn start_program_loop(self: Arc<Self>) {
        tokio::spawn(async move {
            Arc::clone(&self).program_update_loop().await;
        });
    }

    /// Arrête le programme en cours
    pub async fn stop_program(&self) -> Result<()> {
        let mut active = self.active_program.lock().unwrap();

        if let Some(state) = active.take() {
            println!("⏹️  Programme '{}' arrêté", state.program_name);
            println!("   Progression : {:.1}% ({}/{}s)",
                     state.progress_percentage(),
                     state.total_elapsed,
                     state.total_duration);
            Ok(())
        } else {
            bail!("Aucun programme en cours d'exécution");
        }
    }

    /// Obtient l'état du programme en cours
    pub async fn get_active_program(&self) -> Option<ProgramExecutionState> {
        self.active_program.lock().unwrap().clone()
    }

    /// Boucle de mise à jour du programme (appelée toutes les secondes)
    async fn program_update_loop(self: Arc<Self>) {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            let should_stop = {
                let mut active = self.active_program.lock().unwrap();

                if let Some(ref mut state) = *active {
                    // Avancer d'une seconde
                    let finished = state.advance(1);

                    if finished {
                        println!("🏁 Programme '{}' terminé !", state.program_name);
                        true
                    } else {
                        // Mettre à jour la puissance si on a changé d'intervalle
                        let current_power = self.data.lock().unwrap().power;
                        if current_power != state.current_power_target {
                            println!("🔄 Changement d'intervalle : {}W → {}W",
                                     current_power, state.current_power_target);
                            if let Some(ref name) = state.current_interval_name {
                                println!("   Intervalle : {}", name);
                            }

                            // Mettre à jour la puissance de manière asynchrone
                            let controller = self.clone();
                            let power = state.current_power_target;
                            tokio::spawn(async move {
                                let _ = controller.set_power(power).await;
                            });
                        }
                        false
                    }
                } else {
                    // Pas de programme actif, arrêter la boucle
                    true
                }
            };

            if should_stop {
                let mut active = self.active_program.lock().unwrap();
                *active = None;
                break;
            }
        }
    }
}
