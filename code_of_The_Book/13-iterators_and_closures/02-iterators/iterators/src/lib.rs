#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// Listing 13-20: Defining the Counter struct and a new function that creates instances of Counter with an initial value of 0 for count
struct Counter {
    counter: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {counter: 0}
    }
}

// Listing 13-21: Implementing the Iterator trait on our Counter struct
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.counter < 5 {
            self.counter += 1;
            Some(self.counter)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    // Listing 13-22: Testing the functionality of the next method implementation
    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);

    }

    // Listing 13-23: Using a variety of Iterator trait methods on our Counter iterator
    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }

    #[test]
    fn it_works() {
        // Listing 13-13: Creating an iterator
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter(); // mut is required for `next()`, but not for `for`

        // Listing 13-14: Using an iterator in a for loop
        // for val in v1_iter {
        //     println!("Got: {}", val);
        // }

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // Listing 13-16: Calling the sum method to get the total of all items in the iterator
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    // Listing 13-18: Calling the map method to create a new iterator and then calling the collect method to consume the new iterator and create a vector
    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    
        assert_eq!(v2, vec![2, 3, 4]);
    }

    // Listing 13-19: Using the filter method with a closure that captures shoe_size
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size: Vec<Shoe> = shoes.into_iter().filter(|shoe| shoe.size == 10).collect();

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
