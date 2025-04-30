impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let mut ascii_coor1: u8 = coordinate1.as_bytes().iter().map(|&b| b as u8).sum();
        let mut ascii_coor2: u8 = coordinate2.as_bytes().iter().map(|&b| b as u8).sum();
        
        ascii_coor1 % 2 == ascii_coor2 % 2
    }
}