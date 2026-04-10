/*!
# cuda-narrative

Narrative construction for agents.

Humans explain themselves. "I did X because Y happened, which reminded me
of Z, and I wanted W." This isn't a justification — it's a narrative.

Agents need narratives too:
- Explaining decisions to humans (transparency)
- Explaining decisions to themselves (metacognition)
- Sharing experience with other agents (communication)
- Learning from stories (narrative memory)

This crate constructs narratives from events, memory, and goals.
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A narrative element — one piece of a story
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NarrativeElement {
    pub text: String,
    pub element_type: ElementType,
    pub confidence: f64,
    pub timestamp: u64,
    pub source: String,    // memory, perception, inference, goal
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementType {
    Event,        // something happened
    Perception,   // something was observed
    Inference,    // something was deduced
    Goal,         // something was intended
    Action,       // something was done
    Outcome,      // what resulted
    Emotion,      // how it felt
    CausalLink,   // A caused B
}

/// A narrative arc — a complete story with structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NarrativeArc {
    pub id: String,
    pub title: String,
    pub elements: Vec<NarrativeElement>,
    pub summary: String,
    pub theme: String,        // recurring pattern
    pub lesson: String,       // what was learned
    pub confidence: f64,
    pub created: u64,
    pub tags: Vec<String>,
}

impl NarrativeArc {
    pub fn new(id: &str, title: &str) -> Self {
        NarrativeArc { id: id.to_string(), title: title.to_string(), elements: vec![], summary: String::new(), theme: String::new(), lesson: String::new(), confidence: 0.5, created: now(), tags: vec![] }
    }

    pub fn add_element(&mut self, element: NarrativeElement) {
        self.elements.push(element);
        // Update confidence from elements
        if !self.elements.is_empty() {
            self.confidence = self.elements.iter().map(|e| e.confidence).sum::<f64>() / self.elements.len() as f64;
        }
    }

    /// Generate summary from elements
    pub fn generate_summary(&mut self) {
        if self.elements.is_empty() { self.summary = "No events recorded.".to_string(); return; }

        let events: Vec<_> = self.elements.iter()
            .filter(|e| e.element_type == ElementType::Event || e.element_type == ElementType::Action)
            .collect();
        let outcomes: Vec<_> = self.elements.iter()
            .filter(|e| e.element_type == ElementType::Outcome)
            .collect();

        let mut summary = String::new();
        if let Some(first) = events.first() {
            summary.push_str(&first.text);
        }
        if events.len() > 1 {
            summary.push_str(" then ");
            if let Some(last) = events.last() {
                summary.push_str(&last.text);
            }
        }
        if let Some(outcome) = outcomes.last() {
            summary.push_str(". Result: ");
            summary.push_str(&outcome.text);
        }
        if summary.is_empty() {
            summary = format!("{} narrative with {} elements.", self.title, self.elements.len());
        }
        self.summary = summary;
    }

    /// Extract themes (repeated words/concepts)
    pub fn extract_themes(&mut self) {
        let mut word_counts: HashMap<String, u32> = HashMap::new();
        for element in &self.elements {
            for word in element.text.split_whitespace() {
                let clean: String = word.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect();
                if clean.len() > 3 { *word_counts.entry(clean).or_insert(0) += 1; }
            }
        }
        // Top 3 themes
        let mut sorted: Vec<_> = word_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        self.theme = sorted.iter().take(3).map(|(w, _)| w.as_str()).collect::<Vec<_>>().join(", ");
    }
}

/// Narrative templates — common story structures
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NarrativeTemplate {
    pub name: String,
    pub structure: Vec<ElementType>,  // expected element sequence
    pub description: String,
}

impl NarrativeTemplate {
    pub fn basic() -> Self {
        NarrativeTemplate { name: "basic".into(), structure: vec![ElementType::Event, ElementType::Perception, ElementType::Inference, ElementType::Action, ElementType::Outcome], description: "Event → Observe → Think → Act → Result".into() }
    }
    pub fn learning() -> Self {
        NarrativeTemplate { name: "learning".into(), structure: vec![ElementType::Goal, ElementType::Action, ElementType::Outcome, ElementType::Inference, ElementType::Emotion], description: "Intend → Act → Result → Learn → Feel".into() }
    }
    pub fn social() -> Self {
        NarrativeTemplate { name: "social".into(), structure: vec![ElementType::Event, ElementType::Perception, ElementType::Emotion, ElementType::Action, ElementType::CausalLink, ElementType::Outcome], description: "Observe → Feel → Act → Explain → Result".into() }
    }
}

/// The narrative engine
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NarrativeEngine {
    pub arcs: HashMap<String, NarrativeArc>,
    pub templates: Vec<NarrativeTemplate>,
    pub current_arc: Option<String>,
    pub max_arcs: usize,
    pub vocabulary: HashMap<String, f64>, // word importance
}

impl NarrativeEngine {
    pub fn new() -> Self {
        let mut templates = vec![NarrativeTemplate::basic(), NarrativeTemplate::learning(), NarrativeTemplate::social()];
        NarrativeEngine { arcs: HashMap::new(), templates, current_arc: None, max_arcs: 100, vocabulary: HashMap::new() }
    }

    /// Start a new narrative arc
    pub fn begin_arc(&mut self, title: &str) -> String {
        let id = format!("arc_{}", now() % 100000);
        let arc = NarrativeArc::new(&id, title);
        self.arcs.insert(id.clone(), arc);
        self.current_arc = Some(id.clone());
        id
    }

    /// Add element to current arc
    pub fn add(&mut self, text: &str, element_type: ElementType, source: &str) {
        let element = NarrativeElement { text: text.to_string(), element_type, confidence: 0.7, timestamp: now(), source: source.to_string() };
        if let Some(arc_id) = &self.current_arc {
            if let Some(arc) = self.arcs.get_mut(arc_id) {
                arc.add_element(element);
            }
        }
        // Update vocabulary
        for word in text.split_whitespace() {
            let clean: String = word.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect();
            if clean.len() > 2 {
                *self.vocabulary.entry(clean).or_insert(0.0) += 0.1;
            }
        }
    }

    /// End current arc, generate summary and themes
    pub fn end_arc(&mut self) -> Option<NarrativeArc> {
        let arc_id = self.current_arc.take()?;
        if let Some(arc) = self.arcs.get_mut(&arc_id) {
            arc.generate_summary();
            arc.extract_themes();
            // Evict oldest if over max
            if self.arcs.len() > self.max_arcs {
                let oldest = self.arcs.iter().min_by_key(|(_, a)| a.created).map(|(k, _)| k.clone())?;
                self.arcs.remove(&oldest);
            }
            return Some(arc.clone());
        }
        None
    }

    /// Find similar past narratives
    pub fn find_similar(&self, query: &str) -> Vec<&NarrativeArc> {
        let query_words: Vec<String> = query.split_whitespace()
            .map(|w| w.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect())
            .filter(|w| w.len() > 2)
            .collect();

        let mut scored: Vec<(&String, &NarrativeArc, f64)> = self.arcs.iter()
            .map(|(id, arc)| {
                let score = query_words.iter()
                    .filter(|w| arc.summary.to_lowercase().contains(w.as_str()) || arc.theme.to_lowercase().contains(w.as_str()))
                    .count() as f64 / query_words.len().max(1) as f64;
                (id, arc, score)
            })
            .filter(|(_, _, s)| *s > 0.0)
            .collect();

        scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        scored.into_iter().map(|(_, arc, _)| arc).collect()
    }

    /// Build explanation: "I did X because..."
    pub fn explain(&self, action: &str, reason: &str, context: &str) -> String {
        format!("I {} because {}. Context: {}.", action, reason, context)
    }

    /// Fleet story — aggregate all arcs into a summary
    pub fn fleet_story(&self) -> String {
        let total = self.arcs.len();
        let completed: usize = self.arcs.values().filter(|a| !a.summary.is_empty()).count();
        format!("Fleet narrative: {} arcs ({} summarized). Themes across fleet experience.", total, completed)
    }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_begin_add_end() {
        let mut engine = NarrativeEngine::new();
        let arc_id = engine.begin_arc("test mission");
        engine.add("detected obstacle", ElementType::Perception, "sensor");
        engine.add("decided to dodge", ElementType::Action, "deliberation");
        engine.add("successfully avoided", ElementType::Outcome, "feedback");
        let arc = engine.end_arc();
        assert!(arc.is_some());
        let arc = arc.unwrap();
        assert_eq!(arc.elements.len(), 3);
        assert!(!arc.summary.is_empty());
    }

    #[test]
    fn test_narrative_summary() {
        let mut arc = NarrativeArc::new("test", "test");
        arc.add_element(NarrativeElement { text: "saw a wall".into(), element_type: ElementType::Event, confidence: 0.8, timestamp: 0, source: "perception".into() });
        arc.add_element(NarrativeElement { text: "turned left".into(), element_type: ElementType::Action, confidence: 0.9, timestamp: 0, source: "decision".into() });
        arc.add_element(NarrativeElement { text: "found exit".into(), element_type: ElementType::Outcome, confidence: 0.7, timestamp: 0, source: "feedback".into() });
        arc.generate_summary();
        assert!(arc.summary.contains("saw a wall"));
        assert!(arc.summary.contains("found exit"));
    }

    #[test]
    fn test_theme_extraction() {
        let mut arc = NarrativeArc::new("test", "test");
        arc.add_element(NarrativeElement { text: "navigation navigation navigation".into(), element_type: ElementType::Event, confidence: 0.8, timestamp: 0, source: "x".into() });
        arc.add_element(NarrativeElement { text: "navigation path".into(), element_type: ElementType::Action, confidence: 0.8, timestamp: 0, source: "x".into() });
        arc.extract_themes();
        assert!(arc.theme.contains("navigation"));
    }

    #[test]
    fn test_find_similar() {
        let mut engine = NarrativeEngine::new();
        let id = engine.begin_arc("cooking");
        engine.add("prepared meal", ElementType::Action, "x");
        engine.add("food was delicious", ElementType::Outcome, "x");
        engine.end_arc();

        let id2 = engine.begin_arc("cooking again");
        engine.add("prepared another meal", ElementType::Action, "x");
        engine.end_arc();

        let similar = engine.find_similar("meal food cooking");
        assert_eq!(similar.len(), 2);
    }

    #[test]
    fn test_explain() {
        let engine = NarrativeEngine::new();
        let explanation = engine.explain("dodged left", "obstacle detected on right", "navigating corridor");
        assert!(explanation.contains("because"));
    }

    #[test]
    fn test_vocabulary() {
        let mut engine = NarrativeEngine::new();
        engine.begin_arc("test");
        for _ in 0..5 { engine.add("navigation path obstacle", ElementType::Event, "x"); }
        assert!(engine.vocabulary.get("navigation").map_or(0.0, |v| v) > 0.4);
    }

    #[test]
    fn test_narrative_confidence() {
        let mut arc = NarrativeArc::new("test", "test");
        arc.add_element(NarrativeElement { text: "a".into(), element_type: ElementType::Event, confidence: 0.8, timestamp: 0, source: "x".into() });
        arc.add_element(NarrativeElement { text: "b".into(), element_type: ElementType::Action, confidence: 0.6, timestamp: 0, source: "x".into() });
        assert!((arc.confidence - 0.7).abs() < 0.01); // average
    }

    #[test]
    fn test_templates() {
        let basic = NarrativeTemplate::basic();
        assert_eq!(basic.structure.len(), 5);
        assert_eq!(basic.structure[0], ElementType::Event);
    }

    #[test]
    fn test_fleet_story() {
        let engine = NarrativeEngine::new();
        let story = engine.fleet_story();
        assert!(story.contains("0 arcs"));
    }

    #[test]
    fn test_eviction() {
        let mut engine = NarrativeEngine::new();
        engine.max_arcs = 2;
        engine.begin_arc("a"); engine.end_arc();
        engine.begin_arc("b"); engine.end_arc();
        engine.begin_arc("c"); engine.end_arc();
        assert!(engine.arcs.len() <= 2);
    }
}
