use kdri::{KettlerConnection, scan_devices};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;
use anyhow::{Result, bail};

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

        for attempt in 1..=3 {
            match self.attempt_connection().await {
                Ok(_) => {
                    println!("✅ Connecté avec succès !");
                    return;
                }
                Err(e) => {
                    eprintln!("⚠️  Tentative {}/3 échouée : {:?}", attempt, e);
                    if attempt < 3 {
                        println!("🔄 Nouvelle tentative dans 3 secondes...");
                        tokio::time::sleep(Duration::from_secs(3)).await;
                    }
                }
            }
        }

        println!("⚠️  Impossible de se connecter pour le moment.");
        println!("   Le serveur continue de fonctionner. Réessai automatique toutes les 30 secondes...");

        // Continuer à essayer en arrière-plan
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
        let connection = Arc::clone(&self.connection);

        let new_conn = tokio::task::spawn_blocking(move || {
            let devices = scan_devices().map_err(|e| anyhow::anyhow!("Scan failed: {:?}", e))?;
            let device = devices.into_iter().last().ok_or_else(|| anyhow::anyhow!("No Kettler device found"))?;
            println!("📱 Appareil trouvé : {}", device.get_name());
            println!("🔗 Connexion en cours...");
            device.connect().map_err(|e| anyhow::anyhow!("Connect failed: {}", e))
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
}
