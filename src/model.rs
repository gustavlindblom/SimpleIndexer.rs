use std::collections::HashMap;

use crate::lexer::Lexer;

type DocumentFrequency = HashMap<String, usize>;
type TermFrequency = HashMap<String, usize>;
type Documents = HashMap<String, Document>;

pub struct Document {
    term_frequency: TermFrequency,
    count: usize,
}

#[derive(Default)]
pub struct Index {
    pub documents: Documents,
    pub document_frequency: DocumentFrequency,
}

impl Index {
    pub fn search_query(&self, query: &[char]) -> Vec<(String, f32)> {
        let mut result = Vec::new();

        let tokens = Lexer::new(query).collect::<Vec<_>>();
        for (name, document) in &self.documents {
            let mut rank = 0f32;
            for token in &tokens {
                rank += compute_term_frequency_score(&token, document)
                    * compute_inverse_document_frequency_score(
                        &token,
                        self.documents.len(),
                        &self.document_frequency,
                    );
            }
            result.push((name.to_string(), rank));
        }

        result.sort_by(|(_, rank1), (_, rank2)| {
            rank1
                .partial_cmp(rank2)
                .expect(&format!("{rank1} and {rank2} are not comparable"))
        });
        result.reverse();

        result
    }

    pub fn add_document(&mut self, document_name: &str, content: &[char]) {
        let mut term_frequency = TermFrequency::new();

        let mut count = 0;
        for token in Lexer::new(content) {
            if let Some(frequency) = term_frequency.get_mut(&token) {
                *frequency += 1;
            } else {
                term_frequency.insert(token, 1);
            }
            count += 1;
        }

        if count == 0 {
            println!("{document_name} got no tokens");
        }

        for token in term_frequency.keys() {
            if let Some(frequency) = self.document_frequency.get_mut(token) {
                *frequency += 1;
            } else {
                self.document_frequency.insert(token.to_string(), 1);
            }
        }

        self.documents.insert(
            document_name.to_string(),
            Document {
                count,
                term_frequency,
            },
        );
    }
}

fn compute_term_frequency_score(term: &str, document: &Document) -> f32 {
    let n = document.count as f32;
    let m = document.term_frequency.get(term).cloned().unwrap_or(0) as f32;

    m / n
}

fn compute_inverse_document_frequency_score(
    term: &str,
    count: usize,
    document_frequency: &DocumentFrequency,
) -> f32 {
    let n = count as f32;
    let m = document_frequency.get(term).cloned().unwrap_or(1) as f32;

    (n / m).log10()
}
