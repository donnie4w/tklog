// Copyright (c) 2024, donnie4w <donnie4w@gmail.com>
// All rights reserved.
// https://github.com/donnie4w/tklog
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

struct TrieNode<V> {
    children: HashMap<String, TrieNode<V>>,
    module: Option<V>,
}

impl<V> TrieNode<V> {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            module: None,
        }
    }
}

pub struct Trie<V> {
    root: TrieNode<V>,
    count: i32,
    cache: HashMap<String, Option<V>>, 
}

impl<V: Clone> Trie<V> {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
            count: 0,
            cache: HashMap::new(),  
        }
    }

    pub fn len(&self) -> i32 {
        self.count
    }

    pub fn insert(&mut self, pattern: &str, module: V) {
        let segments: Vec<&str> = pattern.split("::").collect();
        let mut node = &mut self.root;
        for segment in segments {
            node = node.children.entry(segment.to_string()).or_insert_with(TrieNode::new);
        }
        node.module = Some(module);
        self.count += 1;
    }

    pub fn get(&mut self, input: &str) -> Option<V> {
        if let Some(cached_result) = self.cache.get(input) {
            return cached_result.clone();
        }
        let segments: Vec<&str> = input.split("::").collect();
        let mut node = &self.root;
        let mut last_matched_module: Option<&V> = None;
        for segment in segments {
            if let Some(child) = node.children.get("*") {
                if let Some(ref module) = child.module {
                    last_matched_module = Some(module);
                }
            }
            if let Some(child) = node.children.get(segment) {
                node = child;
                if let Some(ref module) = node.module {
                    last_matched_module = Some(module);
                }
            } else {
                break;
            }
        }
        let result = last_matched_module.cloned();
        self.cache.insert(input.to_string(), result.clone());
        result
    }
}