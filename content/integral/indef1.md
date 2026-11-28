+++
title = "不定積分 大問演習①（4題）"
level = "indefinite"
order = 2
+++

## 第1問（難）

(1) $(x\sin x+\cos x)'$ と $(x\cos x-\sin x)'$ を求めよ。

(2) 不定積分 $\displaystyle\int\frac{x^2}{(x\sin x+\cos x)^2}\,dx$ を求めよ。

(3) 不定積分 $\displaystyle\int\frac{x^2}{(x\cos x-\sin x)^2}\,dx$ を求めよ。

## 第2問（やや難）

(1) $\left\{\log\left(x+\sqrt{x^2+1}\right)\right\}'$ を求めよ。

(2) 不定積分 $\displaystyle\int\sqrt{x^2+1}\,dx$ を求めよ。

(3) $x>0$ で、不定積分 $\displaystyle\int\frac{\sqrt{x^2+1}}x\,dx$ を求めよ。

## 第3問（やや難）

(1) 不定積分 $\displaystyle\int\frac{x^2-1}{x^4+1}\,dx$ を求めよ。

(2) 不定積分 $\displaystyle\int\frac{x^2-1}{x^4+x^2+1}\,dx$ を求めよ。

(3) $a<2$ を満たす実数 $a$ に対し、不定積分 $\displaystyle\int\frac{x^2-1}{x^4+ax^2+1}\,dx$ を求めよ。また、$a=2$ のときの不定積分も求めよ。

## 第4問（やや難）

$\sin x+\cos x>0$ となる範囲で考える。

(1) 不定積分 $\displaystyle\int\frac{\sin x}{\sin x+\cos x}\,dx$ を求めよ。

(2) 不定積分 $\displaystyle\int\frac{\sin^3x}{\sin x+\cos x}\,dx$ を求めよ。

(3) 定積分 $\displaystyle\int_0^{\frac\pi2}\frac{\sin^3x}{\sin x+\cos x}\,dx$ を、(2) を使う方法と使わない方法の 2 通りで求めよ。

@@解答@@

## 第1問

**答え** (1) $x\cos x$、$-x\sin x$ (2) $\dfrac{\sin x-x\cos x}{x\sin x+\cos x}+C$ (3) $\dfrac{x\sin x+\cos x}{x\cos x-\sin x}+C$

**骨子**
1. (2) (1) の $x\cos x$ を作るように分ける: $\frac{x^2}{(x\sin x+\cos x)^2}=\frac x{\cos x}\cdot\frac{x\cos x}{(x\sin x+\cos x)^2}$。後ろの原始関数は $-\frac1{x\sin x+\cos x}$。
2. $\left(\frac x{\cos x}\right)'=\frac{\cos x+x\sin x}{\cos^2x}$ なので、部分積分すると $-\frac x{\cos x(x\sin x+\cos x)}+\int\frac{dx}{\cos^2x}$。$\tan x$ と通分して答え。
3. (3) 同様に $\frac{x^2}{(x\cos x-\sin x)^2}=\frac x{\sin x}\cdot\frac{x\sin x}{(x\cos x-\sin x)^2}$。後ろの原始関数は $\frac1{x\cos x-\sin x}$。$\left(\frac x{\sin x}\right)'=\frac{\sin x-x\cos x}{\sin^2x}$ で部分積分すると $\frac x{\sin x(x\cos x-\sin x)}+\int\frac{dx}{\sin^2x}=\frac x{\sin x(x\cos x-\sin x)}-\frac{\cos x}{\sin x}$。通分して答え。


## 第2問

**答え** (1) $\dfrac1{\sqrt{x^2+1}}$ (2) $\dfrac12\left\{x\sqrt{x^2+1}+\log\left(x+\sqrt{x^2+1}\right)\right\}+C$ (3) $\sqrt{x^2+1}-\log\dfrac{1+\sqrt{x^2+1}}x+C$

**骨子**
1. (1) $\frac{1+\frac x{\sqrt{x^2+1}}}{x+\sqrt{x^2+1}}=\frac1{\sqrt{x^2+1}}$。
2. (2) 部分積分 $I=x\sqrt{x^2+1}-\int\frac{x^2}{\sqrt{x^2+1}}dx$。$\frac{x^2}{\sqrt{x^2+1}}=\sqrt{x^2+1}-\frac1{\sqrt{x^2+1}}$ なので $I=x\sqrt{x^2+1}-I+\log\left(x+\sqrt{x^2+1}\right)$。
3. (3) $\frac{\sqrt{x^2+1}}x=\frac{x^2+1}{x\sqrt{x^2+1}}=\frac x{\sqrt{x^2+1}}+\frac1{x\sqrt{x^2+1}}$。前は $\sqrt{x^2+1}$。後ろは $u=\sqrt{x^2+1}$($x\,dx=u\,du$)とおくと $\int\frac{du}{u^2-1}=\frac12\log\frac{u-1}{u+1}$。$\frac{u-1}{u+1}=\frac{(u-1)^2}{u^2-1}=\left(\frac{u-1}x\right)^2$ なので $\log\frac{\sqrt{x^2+1}-1}x=-\log\frac{1+\sqrt{x^2+1}}x$。


## 第3問

**答え** (1) $\dfrac1{2\sqrt2}\log\dfrac{x^2-\sqrt2x+1}{x^2+\sqrt2x+1}+C$ (2) $\dfrac12\log\dfrac{x^2-x+1}{x^2+x+1}+C$ (3) $a<2$ のとき $\dfrac1{2c}\log\dfrac{x^2-cx+1}{x^2+cx+1}+C$($c=\sqrt{2-a}$)、$a=2$ のとき $-\dfrac x{x^2+1}+C$

**骨子**
1. $x\ne0$ で分母分子を $x^2$ で割ると $\frac{1-\frac1{x^2}}{x^2+\frac1{x^2}+a}$。$t=x+\frac1x$ とおくと $dt=\left(1-\frac1{x^2}\right)dx$、$x^2+\frac1{x^2}=t^2-2$ なので $\int\frac{dt}{t^2-(2-a)}$。
2. $a<2$ のとき $c=\sqrt{2-a}>0$ として $\frac1{t^2-c^2}=\frac1{2c}\left(\frac1{t-c}-\frac1{t+c}\right)$。$\frac{t-c}{t+c}=\frac{x^2-cx+1}{x^2+cx+1}$。(1) は $a=0$、(2) は $a=1$ の場合。
3. $a=2$ のとき $\int\frac{dt}{t^2}=-\frac1t=-\frac x{x^2+1}$。
4. $x=0$ を含む範囲でも、答えを微分すると被積分関数に一致することを確かめる($x^2\pm cx+1>0$ なので $\log$ の中は常に正)。


## 第4問

**答え** (1) $\dfrac12\left\{x-\log(\sin x+\cos x)\right\}+C$ (2) $\dfrac12\left\{x-\dfrac{\sin^2x}2-\dfrac{(\sin x+\cos x)^2}4-\dfrac12\log(\sin x+\cos x)\right\}+C$ (3) $\dfrac{\pi-1}4$

**骨子**
1. (1) 相棒 $\int\frac{\cos x}{\sin x+\cos x}dx$ と足すと $x$、引くと $\log(\sin x+\cos x)$(分子が分母の微分)。連立して解く。
2. (2) 相棒 $\int\frac{\cos^3x}{\sin x+\cos x}dx$ と足すと $\int(\sin^2x-\sin x\cos x+\cos^2x)dx=x-\frac{\sin^2x}2$。引くと $\int\frac{(\cos x-\sin x)(1+\sin x\cos x)}{\sin x+\cos x}dx$。
3. $u=\sin x+\cos x$ とおくと $du=(\cos x-\sin x)dx$、$\sin x\cos x=\frac{u^2-1}2$。後ろは $\int\frac{u^2+1}{2u}du=\frac{u^2}4+\frac12\log u$。連立して答え。
4. (3) (2) を使うと、原始関数は $x=\frac\pi2$ で $\frac12\left(\frac\pi2-\frac12-\frac14\right)$、$x=0$ で $\frac12\left(-\frac14\right)$。差は $\frac{\pi-1}4$。使わない方法: $x\to\frac\pi2-x$ で $\sin^3$ と $\cos^3$ が入れかわるので、定積分は相棒と等しく、足した値 $\frac\pi2-\frac12$ の半分。
