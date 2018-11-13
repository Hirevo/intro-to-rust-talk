//
// EPITECH PROJECT, 2018
// rust-talk
// File description:
// 6-aliasing
//

// Some rules about the relationship of aliasing with mutations

fn main() {
    let mut vector = vec![1, 2, 3];

    vector.push(4);
    {
        let _ref = &vector[1];
        // vector.push(5);
    }
    vector.push(6);

    {
        let _ref1 = &vector;
        let _ref2 = &vector;
        // let _ref3 = &mut vector;
    }

    {
        let _ref3 = &mut vector;
        _ref3.push(7);
        // vector.push(8);
    }

    vector.push(9);
}