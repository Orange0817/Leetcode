class Solution {
  int romanToInt(String s) {
    int len = s.length;
    int res = 0;
    var nums = <int>[];
    for (int i = 0; i < len; i++) {
      nums.add(value(s[i]));
    }
    for (int i = 0; i < len - 1; i++) {
      if (nums[i] < nums[i + 1]) {
        res -= nums[i];
      } else {
        res += nums[i];
      }
    }
    res += nums[len - 1];
    return res;
  }

  int value(String char) {
    switch(char) {
      case "I":
        return 1;
      case "V":
        return 5;
      case "X":
        return 10;
      case "L":
        return 50;
      case "C":
        return 100;
      case "D":
        return 500;
      case "M":
        return 1000;
    }
    return 0;
  }
}