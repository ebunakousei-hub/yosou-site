+++
title = "定積分 難問演習（5題）"
level = "definite"
order = 1
+++

## 第1問（難）

(1) $t\ge0$ のとき、次の不等式を示せ。
$$t-\frac{t^2}2\le\log(1+t)\le t-\frac{t^2}2+\frac{t^3}3$$

(2) 極限 $\displaystyle\lim_{n\to\infty}n\int_0^1\log\left(1+\frac{x^2}n\right)dx$ を求めよ。

(3) 極限 $\displaystyle\lim_{n\to\infty}n\left\{n\int_0^1\log\left(1+\frac{x^2}n\right)dx-\frac13\right\}$ を求めよ。

## 第2問（難）

定積分 $\displaystyle\int_{-\pi}^{\pi}\frac{x\sin x}{(1+e^x)(1+\cos^2x)}\,dx$ を求めよ。

## 第3問（最難）

定積分 $\displaystyle\int_0^{\pi}\frac{x}{1+\cos^2x}\,dx$ を求めよ。

## 第4問（難）

0 以上の整数 $n$ に対し
$$I_n=\int_0^{\frac\pi2}\cos^nx\cos nx\,dx,\qquad J_n=\int_0^{\frac\pi2}\cos^nx\sin nx\,dx$$
とする。

(1) 正の整数 $n$ に対し、$\left(\cos^nx\sin nx\right)'$ と $\left(\cos^nx\cos nx\right)'$ を、それぞれ $\cos^{n-1}x$ と $(n+1)x$ の三角関数の積で表せ。

(2) $I_n$ と $J_n$ を求めよ。

(3) 極限 $\displaystyle\lim_{n\to\infty}nJ_n$ を求めよ。

## 第5問（最難）

正の整数 $n$ に対し $\displaystyle S_n=\sum_{k=1}^n\frac1{n+k}$ とする。

(1) $a>0$、$h>0$ のとき、次の不等式を示せ。
$$\frac{2h}{2a+h}\le\int_a^{a+h}\frac{dx}x\le\frac h2\left(\frac1a+\frac1{a+h}\right)$$

(2) 極限 $\displaystyle\lim_{n\to\infty}n(S_n-\log2)$ を求めよ。

@@解答@@

## 第1問

**答え** (1) 下の骨子 (2) $\dfrac13$ (3) $-\dfrac1{10}$

**骨子**
1. (1) $f(t)=\log(1+t)-t+\frac{t^2}2$ は $f'(t)=\frac{t^2}{1+t}\ge0$、$f(0)=0$。$g(t)=t-\frac{t^2}2+\frac{t^3}3-\log(1+t)$ は $g'(t)=\frac{t^3}{1+t}\ge0$、$g(0)=0$。
2. (2) $t=\frac{x^2}n$ を代入して積分すると $\frac1{3n}-\frac1{10n^2}\le\int_0^1\log\left(1+\frac{x^2}n\right)dx\le\frac1{3n}-\frac1{10n^2}+\frac1{21n^3}$。$n$ 倍すると $\frac13$ にはさまれる。
3. (3) 同じ不等式から $-\frac1{10}\le n\left\{n\int\cdots-\frac13\right\}\le-\frac1{10}+\frac1{21n}$。

## 第2問

**答え** $\dfrac{\pi^2}4$

**骨子**
1. $f(x)=\frac{x\sin x}{(1+e^x)(1+\cos^2x)}$ とする。$x\sin x$ と $\cos^2x$ は偶関数なので $f(-x)=\frac{e^x\cdot x\sin x}{(1+e^x)(1+\cos^2x)}$、$f(x)+f(-x)=\frac{x\sin x}{1+\cos^2x}$。
2. よって与式 $=\frac12\int_{-\pi}^{\pi}\frac{x\sin x}{1+\cos^2x}dx=\int_0^\pi\frac{x\sin x}{1+\cos^2x}dx$。
3. $x=\pi-t$ で $\int_0^\pi\frac{x\sin x}{1+\cos^2x}dx=\frac\pi2\int_0^\pi\frac{\sin x}{1+\cos^2x}dx$。$u=\cos x$ で $\int_{-1}^1\frac{du}{1+u^2}=\frac\pi2$($u=\tan\theta$)。答えは $\frac\pi2\cdot\frac\pi2$。

## 第3問

**答え** $\dfrac{\pi^2}{2\sqrt2}$

**骨子**
1. $x=\pi-t$ で与式 $=\frac\pi2\int_0^\pi\frac{dx}{1+\cos^2x}=\pi\int_0^{\frac\pi2}\frac{dx}{1+\cos^2x}$。
2. $t=\tan x$ をそのまま使うと $x=\frac\pi2$ で $t$ が定まらない。区間を $\left[0,\frac\pi4\right]$ と $\left[\frac\pi4,\frac\pi2\right]$ に分け、後者は $x=\frac\pi2-y$ で $\left[0,\frac\pi4\right]$ に移す。
3. $\int_0^{\pi/4}\frac{dx}{1+\cos^2x}=\int_0^{\pi/4}\frac{dx/\cos^2x}{\tan^2x+2}=\int_0^1\frac{dt}{t^2+2}$、$\int_0^{\pi/4}\frac{dy}{1+\sin^2y}=\int_0^{\pi/4}\frac{dy/\cos^2y}{1+2\tan^2y}=\int_0^1\frac{dt}{1+2t^2}$。
4. $t=\sqrt2\tan\theta$、$t=\frac1{\sqrt2}\tan\varphi$ とおくと、2 つの和は $\frac1{\sqrt2}(\alpha+\beta)$。ただし $\tan\alpha=\frac1{\sqrt2}$、$\tan\beta=\sqrt2$($0<\alpha,\beta<\frac\pi2$)。
5. $\tan\alpha\tan\beta=1$ なので $\alpha+\beta=\frac\pi2$。よって $\int_0^{\pi/2}\frac{dx}{1+\cos^2x}=\frac\pi{2\sqrt2}$、答えは $\pi\cdot\frac\pi{2\sqrt2}$。

## 第4問

**答え** (1) $\left(\cos^nx\sin nx\right)'=n\cos^{n-1}x\cos(n+1)x$、$\left(\cos^nx\cos nx\right)'=-n\cos^{n-1}x\sin(n+1)x$ (2) $I_n=\dfrac\pi{2^{n+1}}$、$J_n=\dfrac1{2^{n+1}}\displaystyle\sum_{k=1}^n\frac{2^k}k$ (3) $1$

**骨子**
1. (1) 積の微分と加法定理。
2. (2) $\cos(n+1)x+\cos(n-1)x=2\cos x\cos nx$ に $\cos^{n-1}x$ をかけて積分すると、(1) より $\int_0^{\pi/2}\cos^{n-1}x\cos(n+1)x\,dx=\frac1n\left[\cos^nx\sin nx\right]_0^{\pi/2}=0$ なので $2I_n=I_{n-1}$、$I_0=\frac\pi2$。
3. 同様に $\sin(n+1)x+\sin(n-1)x=2\cos x\sin nx$ から $2J_n=J_{n-1}+\frac1n$($\int_0^{\pi/2}\cos^{n-1}x\sin(n+1)x\,dx=-\frac1n\left[\cos^nx\cos nx\right]_0^{\pi/2}=\frac1n$)、$J_0=0$。$2^nJ_n$ の階差から答え。
4. (3) $j=n-k$ とおくと $nJ_n=\sum_{j=0}^{n-1}\frac n{n-j}\cdot\frac1{2^{j+1}}$。$\frac n{n-j}\ge1$ より $nJ_n\ge1-\frac1{2^n}$。
5. 上から: $j\le\frac n2$ では $\frac n{n-j}\le1+\frac{2j}n$、$j>\frac n2$ では $\frac n{n-j}\le n$ かつ $\frac1{2^{j+1}}\le\frac1{2^{n/2}}$。よって $nJ_n\le1+\frac2n\sum_{j\ge0}\frac j{2^{j+1}}+\frac{n^2}{2^{n/2}}=1+\frac2n+\frac{n^2}{2^{n/2}}$。両側が 1 に近づく。

## 第5問

**答え** (1) 下の骨子 (2) $-\dfrac14$

**骨子**
1. (1) $y=\frac1x$ は下に凸。区間の中点での接線はグラフの下、両端を結ぶ線分はグラフの上にあるので、積分は $\frac h{a+\frac h2}$ 以上、台形の面積以下。
2. (2) 台形の不等式を $a=n+k-1$、$h=1$($k=1,\dots,n$)で足すと $\log2\le S_n+\frac1{4n}$。よって $n(S_n-\log2)\ge-\frac14$。
3. 中点の不等式を $a=n+k-\frac12$、$h=1$ で足すと $S_n\le\int_{n+\frac12}^{2n+\frac12}\frac{dx}x=\log\frac{4n+1}{2n+1}$。
4. $n(S_n-\log2)\le n\log\left(1-\frac1{4n+2}\right)\le-\frac n{4n+2}$($\log(1+t)\le t$)。両側が $-\frac14$ に近づく。
