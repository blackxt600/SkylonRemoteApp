use serde::{Deserialize, Serialize};

/// Représente un intervalle dans un programme d'entraînement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingInterval {
    /// Durée de l'intervalle en secondes
    pub duration_secs: u32,
    /// Puissance cible en watts pour cet intervalle
    pub power_target: u16,
    /// Nom optionnel de l'intervalle (ex: "Échauffement", "Sprint", "Récupération")
    pub name: Option<String>,
}

/// Représente un programme d'entraînement complet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingProgram {
    /// Identifiant unique du programme
    pub id: String,
    /// Nom du programme
    pub name: String,
    /// Description optionnelle
    pub description: Option<String>,
    /// Liste des intervalles composant le programme
    pub intervals: Vec<TrainingInterval>,
}

impl TrainingProgram {
    /// Calcule la durée totale du programme en secondes
    pub fn total_duration(&self) -> u32 {
        self.intervals.iter().map(|i| i.duration_secs).sum()
    }

    /// Vérifie si le programme est valide
    pub fn is_valid(&self) -> bool {
        !self.intervals.is_empty() &&
        self.intervals.iter().all(|i| i.duration_secs > 0 && i.power_target >= 25 && i.power_target <= 400)
    }
}

/// État de l'exécution d'un programme
#[derive(Debug, Clone, Serialize)]
pub struct ProgramExecutionState {
    /// ID du programme en cours d'exécution
    pub program_id: String,
    /// Nom du programme
    pub program_name: String,
    /// Index de l'intervalle actuel (0-based)
    pub current_interval_index: usize,
    /// Temps écoulé dans l'intervalle actuel (en secondes)
    pub elapsed_in_interval: u32,
    /// Temps total écoulé depuis le début du programme (en secondes)
    pub total_elapsed: u32,
    /// Durée totale du programme (en secondes)
    pub total_duration: u32,
    /// Puissance cible actuelle
    pub current_power_target: u16,
    /// Nom de l'intervalle actuel
    pub current_interval_name: Option<String>,
    /// Programme complet pour référence
    pub program: TrainingProgram,
}

impl ProgramExecutionState {
    pub fn new(program: TrainingProgram) -> Self {
        let total_duration = program.total_duration();
        let current_power_target = program.intervals.first()
            .map(|i| i.power_target)
            .unwrap_or(0);
        let current_interval_name = program.intervals.first()
            .and_then(|i| i.name.clone());

        Self {
            program_id: program.id.clone(),
            program_name: program.name.clone(),
            current_interval_index: 0,
            elapsed_in_interval: 0,
            total_elapsed: 0,
            total_duration,
            current_power_target,
            current_interval_name,
            program,
        }
    }

    /// Avance le temps d'exécution et change d'intervalle si nécessaire
    /// Retourne true si le programme est terminé
    pub fn advance(&mut self, seconds: u32) -> bool {
        self.elapsed_in_interval += seconds;
        self.total_elapsed += seconds;

        // Vérifier si on doit passer à l'intervalle suivant
        if let Some(current_interval) = self.program.intervals.get(self.current_interval_index) {
            if self.elapsed_in_interval >= current_interval.duration_secs {
                // Passer à l'intervalle suivant
                self.current_interval_index += 1;
                self.elapsed_in_interval = 0;

                // Mettre à jour la puissance cible et le nom
                if let Some(next_interval) = self.program.intervals.get(self.current_interval_index) {
                    self.current_power_target = next_interval.power_target;
                    self.current_interval_name = next_interval.name.clone();
                } else {
                    // Programme terminé
                    return true;
                }
            }
        }

        false
    }

    /// Calcule le pourcentage de progression (0-100)
    pub fn progress_percentage(&self) -> f32 {
        if self.total_duration == 0 {
            return 0.0;
        }
        (self.total_elapsed as f32 / self.total_duration as f32) * 100.0
    }

    /// Temps restant dans le programme (en secondes)
    pub fn remaining_time(&self) -> u32 {
        self.total_duration.saturating_sub(self.total_elapsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_total_duration() {
        let program = TrainingProgram {
            id: "test".to_string(),
            name: "Test Program".to_string(),
            description: None,
            intervals: vec![
                TrainingInterval {
                    duration_secs: 60,
                    power_target: 100,
                    name: Some("Warmup".to_string()),
                },
                TrainingInterval {
                    duration_secs: 120,
                    power_target: 200,
                    name: Some("Work".to_string()),
                },
            ],
        };

        assert_eq!(program.total_duration(), 180);
    }

    #[test]
    fn test_execution_state_advance() {
        let program = TrainingProgram {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: None,
            intervals: vec![
                TrainingInterval {
                    duration_secs: 10,
                    power_target: 100,
                    name: None,
                },
                TrainingInterval {
                    duration_secs: 10,
                    power_target: 200,
                    name: None,
                },
            ],
        };

        let mut state = ProgramExecutionState::new(program);
        assert_eq!(state.current_interval_index, 0);
        assert_eq!(state.current_power_target, 100);

        // Avancer de 5 secondes (toujours dans le premier intervalle)
        let finished = state.advance(5);
        assert!(!finished);
        assert_eq!(state.current_interval_index, 0);
        assert_eq!(state.elapsed_in_interval, 5);

        // Avancer de 5 secondes supplémentaires (passer au deuxième intervalle)
        let finished = state.advance(5);
        assert!(!finished);
        assert_eq!(state.current_interval_index, 1);
        assert_eq!(state.current_power_target, 200);
        assert_eq!(state.elapsed_in_interval, 0);

        // Avancer de 10 secondes (terminer le programme)
        let finished = state.advance(10);
        assert!(finished);
    }
}
