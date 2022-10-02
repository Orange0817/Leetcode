impl Solution {
    pub fn get_value(c: char) -> i32 {
        match c {
            'I' => return 1,
            'V' => return 5,
            'X' => return 10,
            'L' => return 50,
            'C' => return 100,
            'D' => return 500,
            'M' => return 1000,
            _ => return 0,
        }    
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut ret = 0;
        let mut s1 = s.clone();

        let mut temp = "".to_string();
        let mut flag = false;

        while s1.len() > 0 {
            let c = s1.remove(0);

            if temp.len() == 1 {
                temp.push(c);

                flag = false;
                if temp == "CM" {
                    ret += 900;
                } else if temp == "CD" {
                    ret += 400;
                } else if temp == "XC" {
                    ret += 90;
                } else if temp == "XL" {
                    ret += 40;
                } else if temp == "IX" {
                    ret += 9;
                } else if temp == "IV" {
                    ret += 4;
                } else {
                    let cc = temp.remove(0); 
                    ret += Solution::get_value(cc);

                    flag = true;
                }

                if flag == false {
                    temp.remove(0);
                    temp.remove(0);
                }


            } else if c == 'C' || c == 'X' || c == 'I' {
                temp.push(c);
            } else {
                ret += Solution::get_value(c);
            }     
        }

        while temp.len() > 0 {
            let cc = temp.remove(0); 
            ret += Solution::get_value(cc);
        }
        ret
    }
}