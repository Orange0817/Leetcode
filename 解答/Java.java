class Solution {
    public int romanToInt(String s) {
        int result = 0;
        if(s != ""){
            HashMap<String,Integer> map = new HashMap<String,Integer>();
            map.put("I", 1);
            map.put("V", 5);
            map.put("X", 10);
            map.put("L", 50);
            map.put("C", 100);
            map.put("D", 500);
            map.put("M", 1000);

            for(int i = 0 ; i<s.length() ; i++){
                String temp = s.charAt(i)+"";
                int value = 0;
                if(map.containsKey(temp)){
                    value = map.get(temp);
                    if(i != 0 &&  (map.get(s.charAt(i-1)+"") < map.get(temp)) ){
                        value = value - (map.get(s.charAt(i-1)+""))*2;
                    }
                    result = result + value;
                }
            }
        }
        return result;
    }
}