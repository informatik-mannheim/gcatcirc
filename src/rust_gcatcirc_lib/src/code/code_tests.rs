#[cfg(test)]
mod tests {
    use crate::code::CircCode;

    #[test]
    fn new_code_from_string_test() {
        {
            let a = CircCode::new_from_seq("ABCCDE".to_string(), 2).unwrap_or_default();

            assert_eq!(a.alphabet, vec!['A', 'B', 'C', 'D', 'E']);
            assert_eq!(a.tuple_length, vec![2]);
            assert_eq!(a.code, vec!["AB", "CC", "DE"]);
            assert_eq!(a.id, "unknown");
        }
        {
            let a = CircCode::new_from_seq("ABCCDEE".to_string(), 3).unwrap_or_default();

            assert_eq!(a.alphabet, vec!['A', 'B', 'C', 'D', 'E']);
            assert_eq!(a.tuple_length, vec![3]);
            assert_eq!(a.code, vec!["ABC", "CDE"]);
            assert_eq!(a.id, "unknown");
        }
    }

    #[test]
    fn new_code_from_vec_test() {
        {
            let mut a = CircCode::new_from_vec(vec!["BDC".to_string(), "CA".to_string(), "DB".to_string()]).unwrap_or_default();


            assert_eq!(a.tuple_length, vec![2, 3]);
            assert_eq!(a.alphabet, vec!['A', 'B', 'C', 'D']);
            assert_eq!(a.code, vec!["BDC", "CA", "DB"]);
            assert_eq!(a.id, "unknown");
            a.shift(-1);
            assert_eq!(a.code, vec!["CBD", "AC", "BD"]);
            a.shift(1);
            assert_eq!(a.code, vec!["BDC", "CA", "DB"]);
            a.shift(3);
            assert_eq!(a.code, vec!["BDC", "AC", "BD"]);
            a.shift(-4);
            assert_eq!(a.code, vec!["CBD", "AC", "BD"]);
        }

        {
            let res = CircCode::new_from_vec(vec!["BDC".to_string(), "".to_string(), "DB".to_string()]);

            assert_eq!(res.unwrap_err().to_string(), "Empty Word");
        }

        {
            let res = CircCode::new_from_vec(vec![]);

            assert_eq!(res.unwrap_err().to_string(), "Empty Code");
        }
    }

    #[test]
    fn eq_code() {
        let a = CircCode::new_from_vec(vec!["BDC".to_string(), "CA".to_string(), "DB".to_string()]).unwrap_or_default();
        let b = CircCode::new_from_vec(vec!["CA".to_string(), "DB".to_string(), "BDC".to_string()]).unwrap_or_default();
        let c = CircCode::new_from_vec(vec!["C".to_string(), "DB".to_string(), "BDC".to_string()]).unwrap_or_default();
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
        assert_eq!(b.eq(&c), false);
        assert_eq!(a.eq(&c), false);
        assert_eq!(a == b, true);
        assert_eq!(a == c, false);
    }

    #[test]
    fn shift_code() {
        {
            let mut a = CircCode::new_from_vec(vec!["BDC".to_string(), "CA".to_string(), "DB".to_string()]).unwrap_or_default();
            let b = a.clone();

            assert_eq!(a.tuple_length, vec![2, 3]);
            a.shift(-1);
            assert_eq!(a.code, vec!["CBD", "AC", "BD"]);
            a.shift(1);
            assert_eq!(a, b);
            a.shift(3);
            assert_eq!(a.code, vec!["BDC", "AC", "BD"]);
            a.shift(-4);
            assert_eq!(a.code, vec!["CBD", "AC", "BD"]);
        }
    }

    #[test]
    fn is_code() {
        {
            let a = CircCode::new_from_vec(vec!["BDC".to_string(), "CA".to_string(), "DB".to_string()]).unwrap_or_default();
            assert_eq!(a.is_code(), true);
            let a = CircCode::new_from_vec(vec!["ABDC".to_string(), "AB".to_string(), "DC".to_string()]).unwrap_or_default();
            assert_eq!(a.is_code(), false);
        }
    }

    #[test]
    fn ambiguous_sequences_graph() {
        {
            let a = CircCode::new_from_vec(vec!["BDADCC".to_string(), "AD".to_string(), "BD".to_string(), "CC".to_string(), "ADCC".to_string()]).unwrap_or_default();

            let (is_code, an_seq) = a.all_ambiguous_sequences();

            assert_eq!(is_code, false);
            assert_eq!(an_seq, vec!["BDADCC".to_string(), "BDADCC".to_string(), "ADCC".to_string()]);
        }
    }


    #[test]
    fn is_circular() {
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0001".to_string(), "0100".to_string()]).unwrap_or_default();
            let is_circular = a.is_circular();
            assert_eq!(is_circular, false);
        }
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0022".to_string(), "2233".to_string(), "3311".to_string()]).unwrap_or_default();
            let is_circular = a.is_circular();
            assert_eq!(is_circular, false);
        }
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0022".to_string(), "2233".to_string(), "3314".to_string()]).unwrap_or_default();
            let is_circular = a.is_circular();
            assert_eq!(is_circular, true);
        }
    }

    #[test]
    fn is_cn_circular() {
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0001".to_string(), "0100".to_string()]).unwrap_or_default();
            let is_circular = a.is_cn_circular();
            assert_eq!(is_circular, false);
        }
        {
            let mut a = CircCode::new_from_vec(vec!["1100".to_string(), "0022".to_string(), "2233".to_string(), "3311".to_string()]).unwrap_or_default();
            let is_circular = a.is_cn_circular();
            assert_eq!(is_circular, false);
            a.shift(1);
            let is_circular = a.is_cn_circular();
            assert_eq!(is_circular, false);
        }
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0022".to_string(), "2233".to_string(), "3314".to_string()]).unwrap_or_default();
            let is_circular = a.is_cn_circular();
            assert_eq!(is_circular, true);
        }
        {
            let a = CircCode::new_from_vec(vec!["AAC".to_string(), "AAG".to_string(), "AAT".to_string(), "ACC".to_string(), "ACG".to_string(), "ACT".to_string(), "AGC".to_string(), "AGG".to_string(), "AGT".to_string(), "ATT".to_string(), "CCG".to_string(), "CCT".to_string(), "CGG".to_string(), "CGT".to_string(), "CTT".to_string(), "GCT".to_string(), "GGT".to_string(), "GTT".to_string(), "TCA".to_string(), "TGA".to_string()]).unwrap_or_default();
            let is_circular = a.is_cn_circular();
            assert_eq!(is_circular, true);
        }
        {
            // a3({001, 01, 1000}) = {100, 01, 0010} -> non-circular
            let a = CircCode::new_from_vec(vec!["001".to_string(), "01".to_string(),  "1000".to_string()]).unwrap_or_default();

            let is_circular = a.is_cn_circular();
            assert_eq!(is_circular, false);
        }
    }

    #[test]
    fn k_circular() {
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0022".to_string(), "2233".to_string(), "3311".to_string()]).unwrap_or_default();
            let is_circular = a.get_exact_k_circular();
            assert_eq!(is_circular, 1);
        }
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0022".to_string(), "2211".to_string()]).unwrap_or_default();
            let is_circular = a.get_exact_k_circular();
            assert_eq!(is_circular, 2);
        }
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0022".to_string(), "2233".to_string(), "3314".to_string()]).unwrap_or_default();
            let is_circular = a.get_exact_k_circular();
            assert_eq!(is_circular, u32::MAX);
        }

    }

    #[test]
    fn comm_free() {
        {
            let a = CircCode::new_from_vec(vec!["1100".to_string(), "0022".to_string(), "2233".to_string(), "3311".to_string()]).unwrap_or_default();
            assert_eq!(a.is_comma_free(), false);
            assert_eq!(a.is_strong_comma_free(), false);
        }
        {
            let a = CircCode::new_from_vec(vec!["ABC".to_string(), "DEF".to_string()]).unwrap_or_default();
            assert_eq!(a.is_comma_free(), true);
            assert_eq!(a.is_strong_comma_free(), true);
        }
        {
            let a = CircCode::new_from_vec(vec!["ABC".to_string(), "CEF".to_string()]).unwrap_or_default();
            assert_eq!(a.is_comma_free(), true);
            assert_eq!(a.is_strong_comma_free(), false);
        }

    }
}