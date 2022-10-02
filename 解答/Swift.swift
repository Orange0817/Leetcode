class Solution {

    func romanToInt(_ s: String) -> Int {
        let dictionary:[Character: Int] = ["I":1, "V":5, "X":10, "L":50, "C":100, "D":500, "M":1000];

        let chars = Array(s);
        var output = 0;

        if(chars.count == 0) {
            return 0;
        }

        output = dictionary[chars[0]]!

        for i in 1..<chars.count {
            let char = chars[i];
            let value = dictionary[char]!

            let pchar =  chars[i - 1];
            let bvalue = dictionary[pchar]!

            output = output + value;

            if(value > bvalue) {
                output = output - 2 * bvalue;
            }
        }
        return output;
    }
}