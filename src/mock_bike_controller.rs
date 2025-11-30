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
    data: Arc<Mutex<BikeData>>,
    // Stockage des programmes d'entraînement
    programs: Arc<Mutex<HashMap<String, TrainingProgram>>>,
    // État du programme en cours d'exécution
    active_program: Arc<Mutex<Option<ProgramExecutionState>>>,
    // Simulation state
    simulation_running: Arc<Mutex<bool>>,
}

impl BikeController {
    pub async fn new() -> Result<Arc<Self>> {
        println!("🔧 Mode MOCK: Simulation du contrôleur de vélo");
        println!("   Pas de connexion Bluetooth réelle");

        let data = BikeData {
            speed: 0.0,
            rpm: 0,
            power: 100,
            connected: true, // Always connected in mock mode
        };

        let controller = Arc::new(BikeController {
            data: Arc::new(Mutex::new(data)),
            programs: Arc::new(Mutex::new(HashMap::new())),
            active_program: Arc::new(Mutex::new(None)),
            simulation_running: Arc::new(Mutex::new(false)),
        });

        // Start simulation
        let controller_clone = controller.clone();
        tokio::spawn(async move {
            controller_clone.simulate_workout().await;
        });

        println!("✅ Contrôleur mock initialisé avec succès");

        Ok(controller)
    }

    /// Simulates a realistic workout pattern
    async fn simulate_workout(&self) {
        let mut interval = time::interval(Duration::from_secs(1));
        let mut elapsed = 0u32;

        *self.simulation_running.lock().unwrap() = true;

        loop {
            interval.tick().await;
            elapsed += 1;

            let is_running = *self.simulation_running.lock().unwrap();
            if !is_running {
                break;
            }

            // Simulate realistic RPM variations (40-80 RPM)
            let base_rpm = 60;
            let rpm_variation = ((elapsed as f32 * 0.3).sin() * 10.0) as i16;
            let rpm = (base_rpm + rpm_variation).max(0) as u16;

            // Calculate speed based on RPM (rough approximation)
            let speed = (rpm as f32) * 0.18; // ~10.8 km/h at 60 RPM

            let mut data = self.data.lock().unwrap();
            data.rpm = rpm;
            data.speed = speed;
            // power is set by user or program, so we don't modify it here
        }
    }

    pub fn start_polling(self: Arc<Self>, _interval_sec: u64) {
        // In mock mode, polling is already handled by simulate_workout
        println!("🔄 Mode MOCK: Simulation en cours (pas de polling Bluetooth)");
    }

    pub async fn get_data(&self) -> BikeData {
        self.data.lock().unwrap().clone()
    }

    pub async fn get_power(&self) -> u16 {
        self.data.lock().unwrap().power
    }

    pub async fn set_power(&self, level: u16) -> Result<()> {
        if level < 25 || level > 400 {
            bail!("Niveau de puissance hors plage (25-400)");
        }

        let mut data = self.data.lock().unwrap();
        data.power = level;

        println!("⚡ [MOCK] Puissance définie à {}W", level);

        Ok(())
    }

    // ===== Gestion des programmes d'entraînement =====

    pub async fn create_program(&self, program: TrainingProgram) -> Result<()> {
        if !program.is_valid() {
            bail!("Programme invalide : vérifiez que tous les intervalles ont une durée > 0 et une puissance entre 25W et 400W");
        }

        let mut programs = self.programs.lock().unwrap();

        if programs.contains_key(&program.id) {
            bail!("Un programme avec l'ID '{}' existe déjà", program.id);
        }

        println!("📝 [MOCK] Nouveau programme créé : {} ({} intervalles, {}s total)",
                 program.name, program.intervals.len(), program.total_duration());

        programs.insert(program.id.clone(), program);
        Ok(())
    }

    pub async fn update_program(&self, program: TrainingProgram) -> Result<()> {
        if !program.is_valid() {
            bail!("Programme invalide");
        }

        let mut programs = self.programs.lock().unwrap();

        if !programs.contains_key(&program.id) {
            bail!("Programme '{}' introuvable", program.id);
        }

        let active = self.active_program.lock().unwrap();
        if let Some(ref state) = *active {
            if state.program_id == program.id {
                bail!("Impossible de modifier un programme en cours d'exécution");
            }
        }

        println!("📝 [MOCK] Programme mis à jour : {}", program.name);
        programs.insert(program.id.clone(), program);
        Ok(())
    }

    pub async fn delete_program(&self, program_id: &str) -> Result<()> {
        let active = self.active_program.lock().unwrap();
        if let Some(ref state) = *active {
            if state.program_id == program_id {
                bail!("Impossible de supprimer un programme en cours d'exécution");
            }
        }

        let mut programs = self.programs.lock().unwrap();
        if programs.remove(program_id).is_some() {
            println!("🗑️  [MOCK] Programme '{}' supprimé", program_id);
            Ok(())
        } else {
            bail!("Programme '{}' introuvable", program_id);
        }
    }

    pub async fn list_programs(&self) -> Vec<TrainingProgram> {
        let programs = self.programs.lock().unwrap();
        programs.values().cloned().collect()
    }

    pub async fn get_program(&self, program_id: &str) -> Option<TrainingProgram> {
        let programs = self.programs.lock().unwrap();
        programs.get(program_id).cloned()
    }

    pub async fn start_program(&self, program_id: &str) -> Result<()> {
        {
            let active = self.active_program.lock().unwrap();
            if active.is_some() {
                bail!("Un programme est déjà en cours d'exécution. Arrêtez-le d'abord.");
            }
        }

        let program = {
            let programs = self.programs.lock().unwrap();
            programs.get(program_id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Programme '{}' introuvable", program_id))?
        };

        if !program.is_valid() {
            bail!("Programme invalide");
        }

        let state = ProgramExecutionState::new(program);

        self.set_power(state.current_power_target).await?;

        println!("🎯 [MOCK] Démarrage du programme : {}", state.program_name);
        println!("   Durée totale : {}s", state.total_duration);
        println!("   Premier intervalle : {}W", state.current_power_target);

        *self.active_program.lock().unwrap() = Some(state);

        Ok(())
    }

    pub fn start_program_loop(self: Arc<Self>) {
        tokio::spawn(async move {
            Arc::clone(&self).program_update_loop().await;
        });
    }

    pub async fn stop_program(&self) -> Result<()> {
        let mut active = self.active_program.lock().unwrap();

        if let Some(state) = active.take() {
            println!("⏹️  [MOCK] Programme '{}' arrêté", state.program_name);
            println!("   Progression : {:.1}% ({}/{}s)",
                     state.progress_percentage(),
                     state.total_elapsed,
                     state.total_duration);
            Ok(())
        } else {
            bail!("Aucun programme en cours d'exécution");
        }
    }

    pub async fn get_active_program(&self) -> Option<ProgramExecutionState> {
        self.active_program.lock().unwrap().clone()
    }

    async fn program_update_loop(self: Arc<Self>) {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            let should_stop = {
                let mut active = self.active_program.lock().unwrap();

                if let Some(ref mut state) = *active {
                    let finished = state.advance(1);

                    if finished {
                        println!("🏁 [MOCK] Programme '{}' terminé !", state.program_name);
                        true
                    } else {
                        let current_power = self.data.lock().unwrap().power;
                        if current_power != state.current_power_target {
                            println!("🔄 [MOCK] Changement d'intervalle : {}W → {}W",
                                     current_power, state.current_power_target);
                            if let Some(ref name) = state.current_interval_name {
                                println!("   Intervalle : {}", name);
                            }

                            let controller = self.clone();
                            let power = state.current_power_target;
                            tokio::spawn(async move {
                                let _ = controller.set_power(power).await;
                            });
                        }
                        false
                    }
                } else {
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
