let romanNum = {
    I: 1,
    V: 5,
    X: 10,
    L: 50,
    C: 100,
    D: 500,
    M: 1000
}

function romanToInt(s) {
    let strAry = s.split('');
    let sum = 0;
    for (let i = 0; i < strAry.length; i++) {
        if (romanNum[strAry[i]] < romanNum[strAry[i + 1]]) {
            sum -= Number(romanNum[strAry[i]]);
        } else {
            sum += Number(romanNum[strAry[i]]);
        }
    }
    return sum;
}