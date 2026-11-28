+++
title = "不定積分 難問演習（6題）"
level = "indefinite"
order = 2
+++

## 第1問（難）

$0<x<\pi$ で、不定積分 $\displaystyle\int\frac{dx}{\sin x\,(2+\cos x)}$ を求めよ。

## 第2問（難）

$0<x<\dfrac\pi2$ で、不定積分 $\displaystyle\int\left(\sqrt{\tan x}-\frac1{\sqrt{\tan x}}\right)dx$ を求めよ。

## 第3問（難）

$x>0$ で、不定積分 $\displaystyle\int\frac{dx}{x\sqrt{x^2+x+1}}$ を求めよ。

## 第4問（最難）

$x>1$ で、不定積分 $\displaystyle\int\frac{x^2+1}{(x^2-1)\sqrt{x^4+1}}\,dx$ を求めよ。

## 第5問（難）

不定積分 $\displaystyle\int\frac{x^4-1}{x^6+1}\,dx$ を求めよ。

## 第6問（最難）

$-1<x<1$ で、不定積分 $\displaystyle\int\frac{x^2+20}{(x\sin x+5\cos x)^2}\,dx$ を求めよ。

@@解答@@

## 第1問

**答え** $\dfrac16\log(1-\cos x)-\dfrac12\log(1+\cos x)+\dfrac13\log(2+\cos x)+C$

**骨子**
1. 分母分子に $\sin x$ をかけて $\frac{\sin x}{(1-\cos^2x)(2+\cos x)}$。$u=\cos x$ とおくと $du=-\sin x\,dx$ で、$-\int\frac{du}{(1-u)(1+u)(2+u)}$。
2. 部分分数分解: $\frac1{(1-u)(1+u)(2+u)}=\frac{1/6}{1-u}+\frac{1/2}{1+u}-\frac{1/3}{2+u}$(各因数が 0 になる $u$ を代入して係数を出す)。
3. $-\int\frac{du}{1-u}=\log(1-u)$、$-\int\frac{du}{1+u}=-\log(1+u)$、$\int\frac{du}{2+u}=\log(2+u)$ をまとめる。$0<x<\pi$ で $1\pm\cos x>0$。

## 第2問

**答え** $-\sqrt2\log\left(\sin x+\cos x+\sqrt{\sin2x}\right)+C$

**骨子**
1. $\sqrt{\tan x}-\frac1{\sqrt{\tan x}}=\frac{\sin x-\cos x}{\sqrt{\sin x\cos x}}$。
2. $u=\sin x+\cos x$ とおくと $du=(\cos x-\sin x)dx$、$\sin x\cos x=\frac{u^2-1}2$、$1<u\le\sqrt2$。与式 $=-\sqrt2\int\frac{du}{\sqrt{u^2-1}}$。
3. $\left\{\log\left(u+\sqrt{u^2-1}\right)\right\}'=\frac1{\sqrt{u^2-1}}$。$\sqrt{u^2-1}=\sqrt{2\sin x\cos x}=\sqrt{\sin2x}$ に戻す。

## 第3問

**答え** $-\log\dfrac{x+2+2\sqrt{x^2+x+1}}x+C$

**骨子**
1. $x=\frac1t$($t>0$)とおくと $dx=-\frac{dt}{t^2}$、$\sqrt{x^2+x+1}=\frac{\sqrt{t^2+t+1}}t$。与式 $=-\int\frac{dt}{\sqrt{t^2+t+1}}$。
2. $t^2+t+1=\left(t+\frac12\right)^2+\frac34$。$\left\{\log\left(s+\sqrt{s^2+a}\right)\right\}'=\frac1{\sqrt{s^2+a}}$ を $s=t+\frac12$ で使うと $-\log\left(t+\frac12+\sqrt{t^2+t+1}\right)$。
3. $t=\frac1x$ を戻すと $t+\frac12+\sqrt{t^2+t+1}=\frac{x+2+2\sqrt{x^2+x+1}}{2x}$。定数 $\log2$ は $C$ に含める。

## 第4問

**答え** $\dfrac1{2\sqrt2}\log\dfrac{\sqrt{x^4+1}-\sqrt2x}{\sqrt{x^4+1}+\sqrt2x}+C$

**骨子**
1. $x>0$ なので $\sqrt{x^4+1}=x\sqrt{x^2+\frac1{x^2}}$。分母分子を $x^2$ で割ると $\frac{1+\frac1{x^2}}{\left(x-\frac1x\right)\sqrt{x^2+\frac1{x^2}}}$。
2. $t=x-\frac1x$($x>1$ で $t>0$)とおくと $dt=\left(1+\frac1{x^2}\right)dx$、$x^2+\frac1{x^2}=t^2+2$。与式 $=\int\frac{dt}{t\sqrt{t^2+2}}$。
3. $u=\sqrt{t^2+2}$ とおくと $u\,du=t\,dt$ なので $\int\frac{du}{u^2-2}=\frac1{2\sqrt2}\log\frac{u-\sqrt2}{u+\sqrt2}$。
4. $u=\frac{\sqrt{x^4+1}}x$ を戻し、分母分子に $x$ をかける。

## 第5問

**答え** $\dfrac1{2\sqrt3}\log\dfrac{x^2-\sqrt3x+1}{x^2+\sqrt3x+1}+C$

**骨子**
1. $x^6+1=(x^2+1)(x^4-x^2+1)$、$x^4-1=(x^2+1)(x^2-1)$ なので、被積分関数 $=\frac{x^2-1}{x^4-x^2+1}$。
2. $x\ne0$ で分母分子を $x^2$ で割り、$t=x+\frac1x$ とおくと $dt=\left(1-\frac1{x^2}\right)dx$、$x^2-1+\frac1{x^2}=t^2-3$。$\int\frac{dt}{t^2-3}=\frac1{2\sqrt3}\log\left|\frac{t-\sqrt3}{t+\sqrt3}\right|$。
3. $\frac{t-\sqrt3}{t+\sqrt3}=\frac{x^2-\sqrt3x+1}{x^2+\sqrt3x+1}$(分母・分子とも常に正)。答えを微分すると $x=0$ を含めて被積分関数に一致する。

## 第6問

**答え** $\dfrac{5\sin x-x\cos x}{x\sin x+5\cos x}+C$

**骨子**
1. $D=x\sin x+5\cos x$ の相棒として $E=5\sin x-x\cos x$ を考える。$D'=x\cos x-4\sin x=-(E-\sin x)$、$E'=x\sin x+4\cos x=D-\cos x$。
2. $\left(\frac ED\right)'=\frac{E'D-ED'}{D^2}=\frac{D^2+E^2-(D\cos x+E\sin x)}{D^2}$。
3. $D^2+E^2=(x^2+25)(\sin^2x+\cos^2x)=x^2+25$、$D\cos x+E\sin x=5$。よって分子は $x^2+20$ で、$\frac ED$ が原始関数。
4. 発見の筋道: $D$ と $E$ は $(x,5)$ と $(\sin x,\cos x)$ の内積と外積にあたり、$D^2+E^2=x^2+25$ が成り立つ。被積分関数の分子 $x^2+20$ が $x^2+25$ に近いことから、$\frac ED$ の形を予想する。
