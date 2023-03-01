use rand::{Rng, RngCore};

pub trait RngUtil {
    /// slice에서 임의의 요소 1개를 pick합니다. 슬라이스가 비어있는 경우 panic이 발생합니다.
    fn pick_one_from_slice<'a, T>(&mut self, slice: &'a [T]) -> &'a T;
}

impl<C> RngUtil for C
where
    C: RngCore,
{
    #[inline]
    fn pick_one_from_slice<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        // 만약 slice가 비어있다면 gen_range에서 panic
        let pick_index = self.gen_range(0..slice.len());
        unsafe { slice.get_unchecked(pick_index) }
    }
}

#[test]
fn pick_one_test() {
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    let mut rng = SmallRng::from_entropy();
    let v1 = vec![1];
    let v2 = vec![1, 2];

    let pick = *rng.pick_one_from_slice(&v1);
    assert_eq!(pick, 1);

    let pick = *rng.pick_one_from_slice(&v2);
    assert!(pick == 1 || pick == 2);
}

#[test]
#[should_panic(expected = "cannot sample empty range")]
fn pick_one_panic_test() {
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    let mut rng = SmallRng::from_entropy();
    let v1: Vec<usize> = vec![];

    let _pick = *rng.pick_one_from_slice(&v1);
}
