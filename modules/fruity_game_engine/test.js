const test = {
    myprop: 1,
    get prop() {
        return this.myprop;
    },
    set prop(val) {
        this.myprop = val;
    }
};

test.prop = "val";

console.log(test);
console.log(Object.keys(test));