//! Configurable priority queue implementation
use std::collections::HashMap as Map;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use crate::queue::FIFOQueue;
use crate::queue::Queue;

pub struct Feature<T, K>
where
    T: Display + Debug,
    K: Hash + Eq + Display + Debug,
{
    name: String,
    weights: Map<K, u32>,
    getter: fn(T) -> K,
}

impl<T, K> Feature<T, K>
where
    T: Display + Debug,
    K: Hash + Eq + Display + Debug,
{
    fn new(name: String, weights: Map<K, u32>, getter: fn(T) -> K) -> Self {
        Self {
            name,
            weights,
            getter,
        }
    }
}

pub struct PriorityQueueConfig<T, K>
where
    T: Display + Debug,
    K: Hash + Eq + Display + Debug + Clone,
{
    features: Map<String, Feature<T, K>>,
}

impl<T, K> PriorityQueueConfig<T, K>
where
    T: Display + Debug,
    K: Hash + Eq + Display + Debug + Clone,
{
    pub fn new() -> Self {
        Self {
            features: Map::new(),
        }
    }

    pub fn add_feature(&mut self, name: String, weights: Map<K, u32>, getter: fn(T) -> K) { 
        self.features
            .insert(name.clone(), Feature::new(name.clone(), weights, getter));
    }


    pub fn get_feature(&self, name: &str) -> Option<&Feature<T, K>> {
        self.features.get(name)
    }

    pub fn features_as_iter(&self) -> impl Iterator<Item = &Feature<T, K>> {
        self.features.values()
    }

    pub fn clone(&self) -> Self {
        let mut config = Self::new();
        for feature in self.features_as_iter() {
            config.add_feature(feature.name.clone(), feature.weights.clone(), feature.getter);
        }
        config
    }

    pub fn print(&self) {
        for feature in self.features_as_iter() {
            println!("Feature: {}", feature.name);
            println!("Weights:");
            for (key, weight) in feature.weights.iter() {
                println!("{}: {}", key, weight);
            }
        }
    }
}

pub struct PriorityQueue<T, K>
where
    T: Display + Debug + Clone,
    K: Hash + Eq + Display + Debug + Clone,
{
    config: PriorityQueueConfig<T, K>,
    queues: Map<String, FIFOQueue<T>>,
}

impl<T, K> PriorityQueue<T, K>
where
    T: Display + Debug + Clone,
    K: Hash + Eq + Display + Debug + Clone,
{
    pub fn new(config: PriorityQueueConfig<T, K>) -> Self {
        let mut queues = Map::new();
        for feature in config.features_as_iter() {
            queues.insert(feature.name.clone(), FIFOQueue::new());
        }
        Self { config, queues }
    }

    pub fn add(&mut self, elem: T) {
        for feature in self.config.features_as_iter() {
            let key = (feature.getter)(elem.clone());
            let weight = feature.weights.get(&key).unwrap_or(&0);
            let queue = self.queues.get_mut(&feature.name).unwrap();
            for _ in 0..*weight {
                queue.add(elem.clone());
            }
        }
    }

    // Removes the element with the highest priority and returns it. Factors in the weights of each feature.
    pub fn pop(&mut self) -> Option<T> { 
        let mut max_weight = 0;
        let mut max_queue_name = None;
        for feature in self.config.features_as_iter() {
            let queue = self.queues.get(&feature.name).unwrap();
            let weight = queue.len();
            if weight > max_weight {
                max_weight = weight;
                max_queue_name = Some(&feature.name);
            }
        }
        self.queues.get_mut(max_queue_name.unwrap()).unwrap().get()
    }

    pub fn len(&self) -> usize {
        self.queues.values().map(|q| q.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn dump(&mut self, other: &mut Self) {
        for feature in self.config.features_as_iter() {
            let queue = self.queues.get_mut(&feature.name).unwrap();
            let other_queue = other.queues.get_mut(&feature.name).unwrap();
            queue.dump(other_queue);
        }
    }

    // Debug function to print the config, queues, features, weights, and elements in the queue in an easy to read format.
    pub fn print(&self) {
        self.config.print();
        for feature in self.config.features_as_iter() {
            let queue = self.queues.get(&feature.name).unwrap();
            println!("Queue: {}; {}", feature.name, queue);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut config = PriorityQueueConfig::new();
        config.add_feature(
            "feature1".to_string(),
            vec![("a".to_string(), 1), ("b".to_string(), 2)]
                .into_iter()
                .collect(),
            |elem: &str| elem.to_string(),
        );
        config.add_feature(
            "feature2".to_string(),
            vec![("a".to_string(), 2), ("b".to_string(), 1)]
                .into_iter()
                .collect(),
            |elem: &str| elem.to_string(),
        );
        let mut queue = PriorityQueue::new(config.clone());
        queue.add("a");
        queue.add("b");
        queue.add("b");
        queue.print();
        assert_eq!(queue.pop(), Some("b"));
        assert_eq!(queue.pop(), Some("b"));
        assert_eq!(queue.pop(), Some("a"));
        assert_eq!(queue.pop(), None);
        let mut queue2 = PriorityQueue::new(config.clone());
        queue2.add("a");
        queue2.add("b");
        queue2.add("b");
        queue.dump(&mut queue2);
        assert_eq!(queue2.pop(), Some("b"));
        assert_eq!(queue2.pop(), Some("b"));
        assert_eq!(queue2.pop(), Some("a"));
        assert_eq!(queue2.pop(), None);
    }
}