use std::iter::zip;

pub fn extend_matrix<T>(m1: Vec<Vec<T>>, m2: Vec<Vec<T>>, fill: &T) -> Vec<Vec<T>>
where
    T: Clone,
{
    if m1.is_empty() {
        return m2;
    }
    if m2.is_empty() {
        return m1;
    }

    let m1_width = m1[0].len();
    let m2_width = m2[0].len();

    let mut answ = m1
        .into_iter()
        .map(|mut row| {
            row.extend(vec![fill.clone(); m2_width]);
            row
        })
        .collect::<Vec<Vec<T>>>();

    answ.extend(
        m2.into_iter()
            .map(|row| {
                let mut new_row = vec![fill.clone(); m1_width];
                new_row.extend(row);
                new_row
            })
            .collect::<Vec<Vec<T>>>(),
    );

    answ
}

pub fn prepend_column_and_row<T>(m: Vec<Vec<T>>, fill: &T) -> Vec<Vec<T>>
where
    T: Clone,
{
    let m = m
        .into_iter()
        .map(|row| {
            let mut new_row = vec![fill.clone()];
            new_row.extend(row);
            new_row
        })
        .collect();

    prepend_row(m, fill)
}

pub fn prepend_row<T>(m: Vec<Vec<T>>, fill: &T) -> Vec<Vec<T>>
where
    T: Clone,
{
    if m.is_empty() {
        return m;
    }

    vec![vec![fill.clone(); m[0].len()]]
        .into_iter()
        .chain(m)
        .collect()
}

pub fn append_rows<T>(m: impl Iterator<Item = Vec<Vec<T>>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let matrices: Vec<Vec<Vec<T>>> = m.collect();

    if matrices.is_empty() {
        return vec![];
    }

    let mut answ = Vec::new();

    for r in 0..matrices[0].len() {
        let mut full_row = Vec::new();
        for matrix in matrices.iter() {
            let row = matrix[r].iter().cloned();
            full_row.extend(row);
        }
        answ.push(full_row);
    }

    answ
}

pub fn gapped_array<T>(source: Vec<T>, indices: &[usize], fill: &T, length: usize) -> Vec<T>
where
    T: Clone,
{
    let mut answ: Vec<T> = (0..length).map(|_| fill.clone()).collect();

    for (v, &i) in zip(source, indices) {
        answ[i] = v;
    }

    answ
}
