use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use parser::{RuleType, TokenType};
use token::Token;

pub struct AstNode {
    sub_items: HashMap<RuleType, VecDeque<Box<Any>>>,
    pub rule_type: RuleType,
}

impl AstNode {
    pub fn new(rule_type: RuleType) -> AstNode {
        AstNode {
            sub_items: HashMap::new(),
            rule_type,
        }
    }

    pub fn add(&mut self, rule_type: RuleType, node: Box<Any>) {
        self.sub_items
            .entry(rule_type)
            .or_insert_with(VecDeque::new)
            .push_back(node);
    }

    pub fn remove<T: 'static>(&mut self, rule_type: RuleType) -> T {
        let items = self.sub_items.remove(&rule_type);
        match items {
            Some(mut items) => *items
                .pop_front()
                .unwrap()
                .downcast::<T>()
                .expect("failed to downcast item"),
            None => panic!("could not find item for RuleType::{}", rule_type),
        }
    }

    pub fn remove_or<T: 'static>(&mut self, rule_type: RuleType, default: T) -> T {
        let items = self.sub_items.remove(&rule_type);
        match items {
            Some(mut items) => {
                if items.is_empty() {
                    default
                } else {
                    *items
                        .pop_front()
                        .unwrap()
                        .downcast::<T>()
                        .expect("failed to downcast item")
                }
            }
            None => default,
        }
    }

    pub fn remove_opt<T: 'static>(&mut self, rule_type: RuleType) -> Option<T> {
        let items = self.sub_items.remove(&rule_type);
        match items {
            Some(mut items) => {
                if items.is_empty() {
                    None
                } else {
                    let item = *items
                        .pop_front()
                        .unwrap()
                        .downcast::<T>()
                        .expect("failed to downcast item");
                    Some(item)
                }
            }
            None => None,
        }
    }

    pub fn remove_items<T: 'static>(&mut self, rule_type: RuleType) -> Vec<T> {
        let items = self.sub_items.remove(&rule_type);
        match items {
            Some(items) => items
                .into_iter()
                .map(|item| *item.downcast::<T>().expect("failed to downcast item"))
                .collect::<Vec<T>>(),
            None => Vec::new(),
        }
    }

    pub fn remove_token(&mut self, token_type: TokenType) -> Rc<RefCell<Token>> {
        let rule_type = RuleType::from(token_type);
        self.remove(rule_type)
    }

    pub fn remove_tokens(&mut self, token_type: TokenType) -> Vec<Rc<RefCell<Token>>> {
        let rule_type = RuleType::from(token_type);
        self.remove_items(rule_type)
    }
}
