impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut seq:Vec<char> = s.chars().collect();
        let mut len = seq.len();  
        let mut n = 0;
        let mut abs = 1 as i64;
        let base :i64 = 2;
        let zero = ('0'.to_string()).parse::<i64>().unwrap();
    
        if len == 0{
            return 0 as i32;
        }
        while n < len&&seq[n] == ' '{
            n +=1; 
        }
        seq = seq[n..].to_vec();
        if seq.len() == 0{ 
            return 0 as i32; 
        }
        if seq[0] == '-'{ 
            abs = -1;
            seq = seq[1..].to_vec();
        }else if seq[0] == '+'{
            seq = seq[1..].to_vec();
        } 
        len = seq.len();
        n = 0;
        let mut res = 0 as i64;
        while n < len && seq[n] >= '0'&&seq[n] <= '9'{
            let value = (seq[n].to_string()).parse::<i64>().unwrap();
            res = res *10 + (value - zero);
            if abs * res > base.pow(31) - 1{
                return (base.pow(31) - 1) as i32;
            }else if abs * res <  -base.pow(31){
                return (-base.pow(31)) as i32;
            }
            n +=1 ;
        }
        res = abs * res;
        res as i32
    }
}
