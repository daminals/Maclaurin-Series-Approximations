# Maclaurin Series

An expirement for Calc 2 (AMS 161) to see Maclaurin approximations in action. After being told that the way modern calculators find the solution for sin(x), cos(x) and e^x is through the Maclaurin series, I was interested in implementing my own!

The end result ended up being accurate to 11 digits before overflow errors occured!

## Issues Encountered

A big issue was the factorial overflow error. Since factorials quickly become very big numbers they create an overflow error within the first 10 values for n (at least in sine maclaurins), leading to an accuracy of only 4 digits. I combatted this by using an optimized factorial function.

Another issue that occured was different number types having a poor time interacting with each other. I learned about type casting in rust to convert between integers floats and unsigned integers to get accurate approximations


## Formulas

<img src="https://latex.codecogs.com/svg.image?cos(x)&space;=&space;$\sum_{i=0}^{\infty}&space;\frac{(-1)^nx^{2n}}{(2n)!}$&space;\\&space;\\" title="cos(x) = $\sum_{i=0}^{\infty} \frac{(-1)^nx^{2n}}{(2n)!}$ \\ \\" />


<img src="https://latex.codecogs.com/svg.image?sin(x)&space;=&space;$\sum_{i=0}^{\infty}&space;\frac{(-1)^nx^{2n&plus;1}}{(2n&plus;1)!}$&space;\\&space;\\" title="sin(x) = $\sum_{i=0}^{\infty} \frac{(-1)^nx^{2n+1}}{(2n+1)!}$ \\ \\" />


<img src="https://latex.codecogs.com/svg.image?$e^x$&space;=&space;$\sum_{i=0}^{\infty}&space;\frac{x^{n}}{n!}$&space;\\&space;" title="$e^x$ = $\sum_{i=0}^{\infty} \frac{x^{n}}{n!}$ \\ " />