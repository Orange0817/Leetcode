class Solution {

    /**
     * @param String $s
     * @return Integer
     */
    function romanToInt($s) {
        $ss = str_replace(['CM','CD','XC','XL','IX','IV','M','D','C','L','X','V','I'],[',900,',',400,',',90,',',40,',',9,',',4,',',1000,',',500,',',100,',',50,',',10,',',5,',',1,'],$s);
        return array_sum(array_filter(explode(',', $ss)));
    }
}