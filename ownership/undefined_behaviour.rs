fn main() {}

// Note: accessing an out-of-bounds index in an array or vector is NOT undefined behavior in Rust.
// It results in a runtime panic, which safely terminates the program without causing undefined behavior.
fn access_out_of_bounds() {
    let arr = [1, 2, 3];
    // This will cause a panic at runtime, but it is not undefined behavior.
    println!("{}", arr[10]);
}

// Note: an example that there is no undefined behavior, but the borrow checker still rejects the code.
//
// This code will not compile because the borrow checker detects potential undefined behavior,
// even though in this specific case there is none.
struct TestResult {
    // Student's scores on a test
    scores: Vec<usize>,
    // A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {
    pub fn get_curve(&self) -> &Option<usize> {
        &self.curve
    }
    // If there is a curve, then increments all scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.get_curve() {
            // compile error due to borrow checker rules.
            //
            // get_curve() returns an immutable reference to self.curve,
            // the **lifetime** of the returned immutable reference to curve is tied to &self,
            // which is an immutable borrow of self.
            // so we cannot mutably borrow self to modify self.scores while curve is still in scope.
            // the borrow checker thinks the mutable borrow of self might change the heap allocation of self,
            // causing the immutable reference self to become dangling, leading to undefined behavior.
            //
            // however, there is no undefined behavior.
            //
            // it's a limitation of the borrow checker that it cannot analyze this code deeply enough to see that
            // curve is only used to read its value, and does not affect self.scores in any way that would cause undefined behavior.
            for score in self.scores.iter_mut() {
                *score += *curve;
            }
        }
    }
}