+++
title = "定積分 演習①（10問）"
level = "definite"
note = "数学の演習 ／ 定積分 演習①（軸: 計算する前に、式の形を見抜く） ／ 数学Ⅲの範囲で解ける10問"
order = 1
+++

**約束** すべて高校数学(数学Ⅲまで)の範囲で解ける。複素数の積分、重積分、級数展開は使わない。$\log$ は自然対数。答えに $\pi$ が出る問題は、$x=\tan\theta$ の置換で処理できる。

## 問題1（標準）

定積分 $\displaystyle\int_{-1}^{1}\frac{dx}{(1+e^x)(1+x^2)}$ を求めよ。

## 問題2（標準）

定積分 $\displaystyle\int_0^{\pi}\frac{x\sin x}{1+\cos^2x}\,dx$ を求めよ。

## 問題3（標準）

$f(x)=x^3+x$ の逆関数を $g(x)$ とする。定積分 $\displaystyle\int_0^{10}g(x)\,dx$ を求めよ。

## 問題4（標準）

$n$ を正の整数とする。定積分 $\displaystyle\int_0^{n\pi}x\,|\sin x|\,dx$ を求めよ。

## 問題5（標準）

(1) 定積分 $\displaystyle\int_0^1\frac{x^4(1-x)^4}{1+x^2}\,dx$ を求めよ。

(2) $\dfrac1{1260}<\dfrac{22}7-\pi<\dfrac1{630}$ を示せ。

## 問題6（やや難）

定積分 $\displaystyle\int_{\frac12}^{2}\frac1x\sin\left(x-\frac1x\right)dx$ を求めよ。

## 問題7（やや難）

定積分 $\displaystyle\int_0^{2\pi}\frac{dx}{2+\cos x}$ を求めよ。

## 問題8（やや難）

定積分 $\displaystyle\int_0^1\frac{\log(1+x)}{1+x^2}\,dx$ を求めよ。

## 問題9（やや難）

0 以上の整数 $n$ に対して $I_n=\displaystyle\int_0^1(1-x^2)^n\,dx$ とする。

(1) $n\ge1$ のとき、$I_n$ を $I_{n-1}$ で表せ。

(2) 次の等式を示せ。
$$\sum_{k=0}^{n}\frac{(-1)^k{}_n\mathrm C_k}{2k+1}=\frac{(2^n\,n!)^2}{(2n+1)!}$$

## 問題10（難）

0 以上の整数 $n$ に対して $J_n=\displaystyle\int_0^{\frac\pi2}\sin^nx\,dx$ とする。

(1) $n\ge1$ のとき、$nJ_nJ_{n-1}=\dfrac\pi2$ を示せ。

(2) $\displaystyle\lim_{n\to\infty}\sqrt n\,J_n$ を求めよ。

@@解答@@

### 進め方の目安

1 問 15〜25 分。最初の数分は手を動かさずに「この式は何と組になるか」「置換すると何が起きるか」を考える。問題1〜5 で道具をそろえ、問題6〜10 で組み合わせる。

## 問題1

**答え** $\dfrac\pi4$

**骨子**
1. 与式を $I$ とし、$x=-t$ と置換すると $I=\int_{-1}^1\frac{e^t}{(1+e^t)(1+t^2)}dt$。
2. 元の式と足すと、$\frac1{1+e^x}+\frac{e^x}{1+e^x}=1$ なので $2I=\int_{-1}^1\frac{dx}{1+x^2}$。
3. $x=\tan\theta$ とおくと $\int_{-1}^1\frac{dx}{1+x^2}=\int_{-\pi/4}^{\pi/4}d\theta=\frac\pi2$。よって $I=\frac\pi4$。

**ねらい** $e^x$ の部分は原始関数が求まらない。「$x\to-x$ で対になる式と足すと消える」と見抜けるか。積分区間が原点対称なら、まずこの置換を試す。

## 問題2

**答え** $\dfrac{\pi^2}4$

**骨子**
1. $x=\pi-t$ と置換すると $I=\int_0^\pi\frac{(\pi-t)\sin t}{1+\cos^2t}dt$。元の式と足して $2I=\pi\int_0^\pi\frac{\sin x}{1+\cos^2x}dx$。
2. $u=\cos x$ とおくと $\int_0^\pi\frac{\sin x}{1+\cos^2x}dx=\int_{-1}^1\frac{du}{1+u^2}=\frac\pi2$(問題1 の 3)。
3. よって $I=\frac\pi2\cdot\frac\pi2=\frac{\pi^2}4$。

**ねらい** 邪魔な $x$ を「$\pi-x$ との平均」で定数 $\frac\pi2$ に置きかえる。$\sin(\pi-x)=\sin x$、$\cos^2(\pi-x)=\cos^2x$ と、式の残りが変わらないことを確かめるのが要点。

## 問題3

**答え** 14

**骨子**
1. $f(0)=0$、$f(2)=10$ で、$f$ は増加。$g(x)$ の $0\le x\le10$ のグラフは、$y=f(x)$ の $0\le x\le2$ のグラフを直線 $y=x$ で折り返したもの。
2. 面積で考えると $\int_0^{10}g(x)\,dx+\int_0^2f(x)\,dx=2\cdot10$(縦 10・横 2 の長方形)。
3. $\int_0^2(x^3+x)\,dx=4+2=6$ なので、答えは $20-6=14$。

**ねらい** 逆関数は式で書けない(3 次方程式を解く必要がある)。それでも「図形として見れば長方形から引くだけ」と気づけるか。置換 $x=f(t)$ でも同じ結果になる。

## 問題4

**答え** $n^2\pi$

**骨子**
1. $x=n\pi-t$ と置換すると、$|\sin(n\pi-t)|=|\sin t|$ なので $I=\int_0^{n\pi}(n\pi-t)|\sin t|\,dt$。
2. 足して $2I=n\pi\int_0^{n\pi}|\sin x|\,dx=n\pi\cdot2n$。
3. よって $I=n^2\pi$。

**ねらい** 問題2 と同じ「端から測り直す」置換。区間ごとに部分積分するより短い。$|\sin x|$ の周期が $\pi$ であることを使う。

## 問題5

**答え** (1) $\dfrac{22}7-\pi$ (2) 下の骨子

**骨子**
1. (1) 分子を $1+x^2$ で割ると $\frac{x^4(1-x)^4}{1+x^2}=x^6-4x^5+5x^4-4x^2+4-\frac4{1+x^2}$。
2. 積分して $\frac17-\frac23+1-\frac43+4-\pi=\frac{22}7-\pi$。
3. (2) $0<x<1$ で $\frac12<\frac1{1+x^2}<1$ なので、(1) の積分は $\frac12\int_0^1x^4(1-x)^4dx$ と $\int_0^1x^4(1-x)^4dx=\frac1{630}$ の間にある。

**ねらい** 正の関数の積分が「$\frac{22}7-\pi$」になるので、$\pi<\frac{22}7$ が証明できる。積分を「不等式を作る道具」として使う。$\int_0^1x^4(1-x)^4dx$ は展開するか、問題9 と同じ部分積分で求める。

## 問題6

**答え** 0

**骨子**
1. $x=\frac1t$ と置換する。$dx=-\frac{dt}{t^2}$、$x:\frac12\to2$ のとき $t:2\to\frac12$。
2. $I=\int_2^{1/2}t\sin\left(\frac1t-t\right)\left(-\frac{dt}{t^2}\right)=\int_{1/2}^2\frac1t\sin\left(\frac1t-t\right)dt=-I$。
3. よって $I=0$。

**ねらい** 積分区間 $\left[\frac12,2\right]$ が $x\to\frac1x$ で自分自身に移ることに気づくか。$x-\frac1x$ は符号が変わるので、奇関数の積分と同じしくみで 0 になる。

## 問題7

**答え** $\dfrac{2\pi}{\sqrt3}$

**骨子**
1. $\cos(2\pi-x)=\cos x$ なので、与式 $=2\int_0^\pi\frac{dx}{2+\cos x}$。さらに $x=\pi-t$ で $[\frac\pi2,\pi]$ の部分を移すと、$\int_0^\pi=\int_0^{\pi/2}\left(\frac1{2+\cos x}+\frac1{2-\cos x}\right)dx$。
2. $t=\tan\frac x2$($0\le t\le1$)とおくと $\cos x=\frac{1-t^2}{1+t^2}$、$dx=\frac{2\,dt}{1+t^2}$ で、$\int_0^{\pi/2}\frac{dx}{2+\cos x}=\int_0^1\frac{2\,dt}{3+t^2}$、$\int_0^{\pi/2}\frac{dx}{2-\cos x}=\int_0^1\frac{2\,dt}{1+3t^2}$。
3. $t=\sqrt3\tan\theta$、$t=\frac1{\sqrt3}\tan\theta$ でそれぞれ $\frac{\pi}{3\sqrt3}$、$\frac{2\pi}{3\sqrt3}$。合計 $\frac\pi{\sqrt3}$ を 2 倍する。

**ねらい** $t=\tan\frac x2$ をいきなり $[0,2\pi]$ に使うと、$x=\pi$ で $t$ が定義できず誤る。**置換が使える区間に区切ってから置換する**のが本質。区切り方を工夫すると、変換後の積分区間が $[0,1]$ に収まる。

## 問題8

**答え** $\dfrac\pi8\log2$

**骨子**
1. $x=\tan\theta$ とおくと、与式 $=\int_0^{\pi/4}\log(1+\tan\theta)\,d\theta$。
2. $\theta=\frac\pi4-u$ とおくと $1+\tan\left(\frac\pi4-u\right)=1+\frac{1-\tan u}{1+\tan u}=\frac2{1+\tan u}$。
3. よって $I=\int_0^{\pi/4}\{\log2-\log(1+\tan u)\}du=\frac\pi4\log2-I$。$I=\frac\pi8\log2$。

**ねらい** 原始関数は高校の範囲では求まらない。$x=\tan\theta$ で三角関数に直し、さらに「端から測り直す」置換で自分自身と組にする 2 段構え。問題1・2 の考え方の発展。

## 問題9

**答え** (1) $I_n=\dfrac{2n}{2n+1}I_{n-1}$ (2) 下の骨子

**骨子**
1. (1) $I_n=\int_0^1(1-x^2)^n\cdot1\,dx$ を部分積分すると $I_n=\left[x(1-x^2)^n\right]_0^1+2n\int_0^1x^2(1-x^2)^{n-1}dx$。$x^2=1-(1-x^2)$ と書いて $I_n=2n(I_{n-1}-I_n)$。
2. (2) 左辺は、$(1-x^2)^n$ を二項定理で展開して積分したもの $\sum_k(-1)^k{}_n\mathrm C_k\int_0^1x^{2k}dx$ に等しい。つまり左辺 $=I_n$。
3. 右辺: (1) をくり返して $I_n=\frac{2n}{2n+1}\cdot\frac{2n-2}{2n-1}\cdots\frac23\cdot I_0=\frac{(2\cdot4\cdots2n)^2}{(2n+1)!}=\frac{(2^nn!)^2}{(2n+1)!}$。

**ねらい** 同じ積分を **2 通りに計算して等式を作る**。直接示すのが難しい二項係数の和が、積分を経由すると一瞬で示せる。

## 問題10

**答え** (1) 下の骨子 (2) $\sqrt{\dfrac\pi2}$

**骨子**
1. (1) 部分積分で $J_n=\frac{n-1}nJ_{n-2}$($n\ge2$)。よって $nJ_nJ_{n-1}=(n-1)J_{n-1}J_{n-2}$ で、$nJ_nJ_{n-1}$ は $n$ によらない。$n=1$ で $1\cdot J_1J_0=1\cdot\frac\pi2$。
2. (2) $0\le\sin x\le1$ なので $J_{n+1}\le J_n\le J_{n-1}$。(1) から $J_nJ_{n+1}=\frac\pi{2(n+1)}$、$J_nJ_{n-1}=\frac\pi{2n}$。
3. よって $\frac\pi{2(n+1)}\le J_n^2\le\frac\pi{2n}$。$nJ_n^2$ は $\frac\pi2$ にはさまれ、$\sqrt nJ_n\to\sqrt{\frac\pi2}$。

**ねらい** 漸化式から「$n$ によらない量」を見つけ、単調性ではさむ。$J_n$ 自体は $n$ の偶奇で式が変わるが、2 つ並べた積は簡単になる、という発想が本質(ウォリスの公式)。
