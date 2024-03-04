import init, {fib} from"./pkg/fib.js";

init().then(()=>{
    console.log(Number(fib(20)));
});