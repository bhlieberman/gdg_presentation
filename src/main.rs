trait Mapper {
    fn map_num(acc: Vec<i8>, num: i8) -> Vec<i8>;
    fn map_str<T>(str: T) -> String;
}

struct Data<T> {
    data: T
}

impl Mapper for Data<i8> {
    fn map_num(mut acc: Vec<i8>, num: i8) -> Vec<i8> {
        acc.push(num + 1);
        acc
    }

    fn map_str<i8>(str: i8) -> String {
        unimplemented!()
    }
}

fn my_map(seq: Vec<i8>) -> Vec<i8> {
    let data: Data<Vec<i8>> = Data { data: seq };
    data.data.into_iter().fold(vec![], Data::map_num)
}

fn main() {
    println!("{:#?}", my_map((1..10).collect()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_my_map() {
	    let nums: Vec<i8> = (1..10).collect();
        assert_eq!((2..11).collect::<Vec<i8>>(), my_map(nums));
    }
}
