+++
title = "定積分 大問演習①（5題）"
level = "definite"
order = 1
+++

## 第1問（やや難）

(1) $t\ge0$ のとき、次の不等式を示せ。
$$t-\frac{t^2}2\le\log(1+t)\le t-\frac{t^2}2+\frac{t^3}3$$

(2) 極限 $\displaystyle\lim_{n\to\infty}n\int_0^1\log\left(1+\frac{x^2}n\right)dx$ を求めよ。

(3) 極限 $\displaystyle\lim_{n\to\infty}n\left\{n\int_0^1\log\left(1+\frac{x^2}n\right)dx-\frac13\right\}$ を求めよ。

## 第2問（標準）

(1) 定積分 $\displaystyle\int_0^{\frac\pi4}\log(1+\tan x)\,dx$ を求めよ。

(2) 定積分 $\displaystyle\int_0^1\frac{\log(1+x)}{1+x^2}\,dx$ を求めよ。

(3) 定積分 $\displaystyle\int_0^{\frac\pi4}\frac{x}{\cos x\,(\sin x+\cos x)}\,dx$ を求めよ。

## 第3問（やや難）

0 以上の整数 $n$ に対し
$$I_n=\int_0^{\frac\pi2}\cos^nx\cos nx\,dx,\qquad J_n=\int_0^{\frac\pi2}\cos^nx\sin nx\,dx$$
とする。

(1) 正の整数 $n$ に対し、$\left(\cos^nx\sin nx\right)'$ と $\left(\cos^nx\cos nx\right)'$ を、それぞれ $\cos^{n-1}x$ と $(n+1)x$ の三角関数の積で表せ。

(2) $I_n$ を求めよ。

(3) $J_n$ を求めよ。

## 第4問（やや難）

0 以上の整数 $n$ に対し $\displaystyle J_n=\frac1{n!}\int_0^1x^ne^{-x}\,dx$ とする。

(1) $n\ge1$ のとき、$J_n$ を $J_{n-1}$ で表せ。

(2) $\displaystyle e-\sum_{k=0}^n\frac1{k!}=eJ_n$ を示せ。さらに $\displaystyle0<e-\sum_{k=0}^n\frac1{k!}<\frac e{(n+1)!}$ を示せ。

(3) $e$ は無理数であることを示せ。

## 第5問（難）

正の整数 $n$ に対し $\displaystyle S_n=\sum_{k=1}^n\frac1{n+k}$ とする。

(1) $a>0$、$h>0$ のとき、次の不等式を示せ。
$$\frac{2h}{2a+h}\le\int_a^{a+h}\frac{dx}x\le\frac h2\left(\frac1a+\frac1{a+h}\right)$$

(2) $\displaystyle-\frac14\le n(S_n-\log2)\le n\log\frac{4n+1}{4n+2}$ を示せ。

(3) 極限 $\displaystyle\lim_{n\to\infty}n(S_n-\log2)$ を求めよ。

@@解答@@

## 第1問

**答え** (1) 下の骨子 (2) $\dfrac13$ (3) $-\dfrac1{10}$

**骨子**
1. (1) $f(t)=\log(1+t)-t+\frac{t^2}2$ は $f'(t)=\frac{t^2}{1+t}\ge0$、$f(0)=0$ なので $f\ge0$。$g(t)=t-\frac{t^2}2+\frac{t^3}3-\log(1+t)$ は $g'(t)=\frac{t^3}{1+t}\ge0$、$g(0)=0$。
2. (2) $t=\frac{x^2}n$ を代入して積分すると $\frac1{3n}-\frac1{10n^2}\le\int_0^1\log\left(1+\frac{x^2}n\right)dx\le\frac1{3n}-\frac1{10n^2}+\frac1{21n^3}$。$n$ 倍すると $\frac13$ にはさまれる。
3. (3) 同じ不等式から $-\frac1{10}\le n\left\{n\int\cdots-\frac13\right\}\le-\frac1{10}+\frac1{21n}$。


## 第2問

**答え** (1) $\dfrac\pi8\log2$ (2) $\dfrac\pi8\log2$ (3) $\dfrac\pi8\log2$

**骨子**
1. (1) $x=\frac\pi4-u$ とおくと $1+\tan\left(\frac\pi4-u\right)=\frac2{1+\tan u}$。よって $I=\int_0^{\pi/4}\{\log2-\log(1+\tan u)\}du=\frac\pi4\log2-I$。
2. (2) $x=\tan\theta$ とおくと $dx=\frac{d\theta}{\cos^2\theta}$、$1+x^2=\frac1{\cos^2\theta}$ で、(1) の積分になる。
3. (3) $\{\log(1+\tan x)\}'=\frac1{\cos^2x(1+\tan x)}=\frac1{\cos x(\sin x+\cos x)}$。部分積分で $\left[x\log(1+\tan x)\right]_0^{\pi/4}-\int_0^{\pi/4}\log(1+\tan x)\,dx=\frac\pi4\log2-\frac\pi8\log2$。


## 第3問

**答え** (1) $\left(\cos^nx\sin nx\right)'=n\cos^{n-1}x\cos(n+1)x$、$\left(\cos^nx\cos nx\right)'=-n\cos^{n-1}x\sin(n+1)x$ (2) $I_n=\dfrac\pi{2^{n+1}}$ (3) $J_n=\dfrac1{2^{n+1}}\displaystyle\sum_{k=1}^n\frac{2^k}k$

**骨子**
1. (1) 積の微分で $n\cos^{n-1}x(\cos x\cos nx-\sin x\sin nx)=n\cos^{n-1}x\cos(n+1)x$。もう一方も同様。
2. (2) 積和の公式 $\cos(n+1)x+\cos(n-1)x=2\cos x\cos nx$ に $\cos^{n-1}x$ をかけて積分すると $\int_0^{\pi/2}\cos^{n-1}x\cos(n+1)x\,dx+I_{n-1}=2I_n$。左の積分は (1) より $\frac1n\left[\cos^nx\sin nx\right]_0^{\pi/2}=0$。よって $I_n=\frac12I_{n-1}$、$I_0=\frac\pi2$。
3. (3) 同様に $\sin(n+1)x+\sin(n-1)x=2\cos x\sin nx$ から $\int_0^{\pi/2}\cos^{n-1}x\sin(n+1)x\,dx+J_{n-1}=2J_n$。左の積分は $-\frac1n\left[\cos^nx\cos nx\right]_0^{\pi/2}=\frac1n$。よって $2^nJ_n=2^{n-1}J_{n-1}+\frac{2^{n-1}}n$、$J_0=0$。


## 第4問

**答え** (1) $J_n=J_{n-1}-\dfrac1{e\cdot n!}$ (2)(3) 下の骨子

**骨子**
1. (1) 部分積分: $\int_0^1x^ne^{-x}dx=\left[-x^ne^{-x}\right]_0^1+n\int_0^1x^{n-1}e^{-x}dx=-\frac1e+n\int_0^1x^{n-1}e^{-x}dx$。両辺を $n!$ で割る。
2. (2) $J_0=1-\frac1e$ と (1) から $J_n=1-\frac1e\sum_{k=0}^n\frac1{k!}$。$e$ 倍する。$0<x^ne^{-x}\le x^n$($0<x\le1$)なので $0<J_n<\frac1{(n+1)!}$。
3. (3) $e=\frac pq$($p,q$ は正の整数、$q\ge2$ としてよい)と仮定し、$n=q$ として (2) を $q!$ 倍する。$q!\left(e-\sum_{k=0}^q\frac1{k!}\right)$ は整数だが、$0$ より大きく $\frac e{q+1}<1$ より小さい。矛盾。


## 第5問

**答え** (1)(2) 下の骨子 (3) $-\dfrac14$

**骨子**
1. (1) $y=\frac1x$ は下に凸。区間の中点での接線はグラフの下、両端を結ぶ線分はグラフの上にあるので、積分は「中点の値 × 幅」$\frac{h}{a+\frac h2}$ 以上、「台形の面積」以下。
2. (2) 左側: 台形の不等式を $a=n+k-1$、$h=1$($k=1,\dots,n$)で足すと $\log2\le\sum_{k=1}^n\frac12\left(\frac1{n+k-1}+\frac1{n+k}\right)=S_n+\frac1{4n}$。
3. 右側: 中点の不等式を $a=n+k-\frac12$、$h=1$ で足すと $S_n\le\int_{n+\frac12}^{2n+\frac12}\frac{dx}x=\log\frac{4n+1}{2n+1}$。$\log2$ を引いて $n$ 倍する。
4. (3) $n\log\frac{4n+1}{4n+2}=n\log\left(1-\frac1{4n+2}\right)\le-\frac n{4n+2}$($\log(1+t)\le t$)。両側が $-\frac14$ に近づく。
