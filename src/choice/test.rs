macro_rules! choice_tests {
    ($module:ident, $word:ty, $bits:expr) => {
        mod $module {
            use super::super::Choice;

            struct FixedRng {
                word: $word,
                draws: usize,
            }

            impl trng::Rng for FixedRng {
                type Word = $word;

                fn nextu(&mut self) -> $word {
                    self.draws += 1;
                    self.word
                }
            }

            #[test]
            fn empty_slice_returns_none_without_drawing() {
                let mut rng = FixedRng { word: 0, draws: 0 };
                assert_eq!(rng.choice::<i32>(&[]), None);
                assert_eq!(rng.draws, 0);
            }

            #[test]
            fn single_item_is_selected_at_both_ends() {
                for word in [0, <$word>::MAX] {
                    let mut rng = FixedRng { word, draws: 0 };
                    assert_eq!(rng.choice(&[42]), Some(&42));
                    assert_eq!(rng.draws, 1);
                }
            }

            #[test]
            fn chooses_first_middle_and_last_items() {
                let items = ["first", "middle", "last"];
                for (word, expected) in [(0, 0), (1 << ($bits - 1), 1), (<$word>::MAX, 2)] {
                    let mut rng = FixedRng { word, draws: 0 };
                    let selected = rng.choice(&items).unwrap();
                    assert!(std::ptr::eq(selected, &items[expected]));
                    assert_eq!(rng.draws, 1);
                }
            }
        }
    };
}

choice_tests!(bits32, u32, 32);
choice_tests!(bits64, u64, 64);
