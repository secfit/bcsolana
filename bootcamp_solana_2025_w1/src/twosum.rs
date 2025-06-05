use std::collections::HashMap;

// Define the two_sum function public
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Create an empty HashMap to store numbers we've seen and their indices
    let mut seen = HashMap::new();

    // Loop through the list using enumerate to get both index and number
    for (i, num) in nums.iter().enumerate() {
        // Calculate the complement: the number we need to reach the target
        let complement = target - num;

        // Check if the complement has already been seen (i.e., exists in the HashMap)
        if let Some(&j) = seen.get(&complement) {
            // If found, return the pair of indices (j from the map, i from current iteration)
            return vec![j as i32, i as i32];
        }

        // If not found, store the current number and its index in the map
        // So that we can find it later if its complement comes up
        seen.insert(num, i);
    }

    // Return empty if no solution found (won't happen as per problem statement)
    vec![]
}
