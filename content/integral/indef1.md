+++
title = "不定積分 演習①（10問）"
level = "indefinite"
note = "数学の演習 ／ 不定積分 演習①（軸: 微分した形を逆に探す） ／ 数学Ⅲの範囲で解ける10問"
order = 2
+++

**約束** すべて高校数学(数学Ⅲまで)の範囲で解ける。$\log$ は自然対数、$C$ は積分定数。$\tan^{-1}$(逆三角関数)を使わずに答えが書ける問題だけを選んだ。各問の $x$ は、式が意味をもつ範囲(分母が 0 でない、$\log$ の中が正)で考える。

## 問題1（易）

$\displaystyle\int\frac{dx}{1+e^x}$ を求めよ。

## 問題2（標準）

$\displaystyle\int\frac{\sin x}{\sin x+\cos x}\,dx$ を求めよ。

## 問題3（標準）

$\displaystyle\int\frac{xe^x}{(1+x)^2}\,dx$ を求めよ。

## 問題4（標準）

$n$ を正の整数とする。$x>0$ で $\displaystyle\int\frac{dx}{x(x^n+1)}$ を求めよ。

## 問題5（標準）

$-\pi<x<\pi$ で $\displaystyle\int\frac{x}{1+\cos x}\,dx$ を求めよ。

## 問題6（標準）

$\displaystyle\int\log\left(x+\sqrt{x^2+1}\right)dx$ を求めよ。

## 問題7（やや難）

$\displaystyle\int\sqrt{x^2+1}\,dx$ を求めよ。

## 問題8（やや難）

$x>0$ で $\displaystyle\int\frac{x+1}{x(1+xe^x)}\,dx$ を求めよ。

## 問題9（やや難）

$\displaystyle\int\frac{x^2-1}{x^4+1}\,dx$ を求めよ。

## 問題10（難）

$\displaystyle\int\frac{x^2}{(x\sin x+\cos x)^2}\,dx$ を求めよ。

@@解答@@

### 進め方の目安

1 問 10〜20 分。不定積分は「答えを微分すれば確かめられる」ので、答えが出たら必ず微分して元に戻るかを確かめる。迷ったら「分母を微分すると何になるか」「この式はどんな関数の微分の形か」を考える。

## 問題1

**答え** $x-\log(1+e^x)+C$

**骨子**
1. $\frac1{1+e^x}=\frac{(1+e^x)-e^x}{1+e^x}=1-\frac{e^x}{1+e^x}$。
2. $\frac{e^x}{1+e^x}$ は「分母の微分 / 分母」なので、積分は $\log(1+e^x)$。

**ねらい** 「1 を足して引く」で、分子に分母の微分を作る。分母分子に $e^{-x}$ をかけて $\frac{e^{-x}}{e^{-x}+1}$ としても同じ答えになる(形は違うが差は定数)。

## 問題2

**答え** $\dfrac12\left(x-\log|\sin x+\cos x|\right)+C$

**骨子**
1. $I=\int\frac{\sin x}{\sin x+\cos x}dx$、$J=\int\frac{\cos x}{\sin x+\cos x}dx$ とおく。
2. $I+J=\int1\,dx=x$。$J-I=\int\frac{\cos x-\sin x}{\sin x+\cos x}dx=\log|\sin x+\cos x|$(分子が分母の微分)。
3. 2 式から $I=\frac12(x-\log|\sin x+\cos x|)+C$。

**ねらい** 1 つだけでは解けない積分を、**相棒の積分と連立して**解く。足すと簡単、引くと分母の微分、という組を見つけられるか。

## 問題3

**答え** $\dfrac{e^x}{1+x}+C$

**骨子**
1. $\left(\frac{e^x}{1+x}\right)'=\frac{e^x(1+x)-e^x}{(1+x)^2}=\frac{xe^x}{(1+x)^2}$。
2. 発見の筋道: $\frac{xe^x}{(1+x)^2}=e^x\left\{\frac1{1+x}-\frac1{(1+x)^2}\right\}$ と分けると、$e^x\{f(x)+f'(x)\}$($f(x)=\frac1{1+x}$)の形。これは $(e^xf(x))'$。

**ねらい** $e^x\{f(x)+f'(x)\}$ の形を見抜く。部分積分で $\frac1{(1+x)^2}$ の側を積分しても同じ結果になり、2 つの方法が同じことをしていると分かると理解が深まる。

## 問題4

**答え** $\dfrac1n\log\dfrac{x^n}{x^n+1}+C$

**骨子**
1. 分母分子に $x^{n-1}$ をかけて $\int\frac{x^{n-1}}{x^n(x^n+1)}dx$。$t=x^n$ とおくと $dt=nx^{n-1}dx$ で、$\frac1n\int\frac{dt}{t(t+1)}$。
2. $\frac1{t(t+1)}=\frac1t-\frac1{t+1}$ より $\frac1n\log\frac t{t+1}+C$。

**ねらい** $n$ が一般なので部分分数分解を直接は使えない。**$x^n$ をひとかたまりにする**ために、わざと $x^{n-1}$ をかける。

## 問題5

**答え** $x\tan\dfrac x2+2\log\left(\cos\dfrac x2\right)+C$

**骨子**
1. $1+\cos x=2\cos^2\frac x2$ なので、与式 $=\int x\cdot\frac1{2\cos^2\frac x2}dx$。
2. $\left(\tan\frac x2\right)'=\frac1{2\cos^2\frac x2}$ を使って部分積分: $x\tan\frac x2-\int\tan\frac x2\,dx$。
3. $\int\tan\frac x2\,dx=-2\log\left(\cos\frac x2\right)$($-\pi<x<\pi$ で $\cos\frac x2>0$)。

**ねらい** 半角の公式で分母を「2 乗の形」にすると、部分積分の相手($\tan\frac x2$)が見える。範囲 $-\pi<x<\pi$ が絶対値を外す根拠になる。

## 問題6

**答え** $x\log\left(x+\sqrt{x^2+1}\right)-\sqrt{x^2+1}+C$

**骨子**
1. $\left\{\log\left(x+\sqrt{x^2+1}\right)\right\}'=\frac{1+\frac x{\sqrt{x^2+1}}}{x+\sqrt{x^2+1}}=\frac1{\sqrt{x^2+1}}$。
2. 部分積分で $x\log\left(x+\sqrt{x^2+1}\right)-\int\frac x{\sqrt{x^2+1}}dx$。後ろは $\sqrt{x^2+1}$。

**ねらい** 複雑に見える $\log$ の中身を微分すると $\frac1{\sqrt{x^2+1}}$ という簡単な形になる。この事実は問題7 の鍵でもある。

## 問題7

**答え** $\dfrac12\left\{x\sqrt{x^2+1}+\log\left(x+\sqrt{x^2+1}\right)\right\}+C$

**骨子**
1. $I=\int\sqrt{x^2+1}\,dx$ を部分積分: $I=x\sqrt{x^2+1}-\int\frac{x^2}{\sqrt{x^2+1}}dx$。
2. $\frac{x^2}{\sqrt{x^2+1}}=\sqrt{x^2+1}-\frac1{\sqrt{x^2+1}}$ なので、$I=x\sqrt{x^2+1}-I+\int\frac{dx}{\sqrt{x^2+1}}$。
3. 問題6 の 1 より $\int\frac{dx}{\sqrt{x^2+1}}=\log\left(x+\sqrt{x^2+1}\right)$。$2I$ について解く。

**ねらい** 部分積分で**同じ積分が右辺に戻ってくる**のを方程式として解く。$t=x+\sqrt{x^2+1}$ と置換する方法もあるが、問題6 の微分を使うと置換なしで済む。

## 問題8

**答え** $\log\dfrac{xe^x}{1+xe^x}+C$(つまり $x+\log x-\log(1+xe^x)+C$)

**骨子**
1. $t=xe^x$ とおくと $dt=(x+1)e^x\,dx$。
2. $\frac{x+1}{x(1+xe^x)}dx=\frac{(x+1)e^x\,dx}{xe^x(1+xe^x)}=\frac{dt}{t(1+t)}$。
3. $\int\frac{dt}{t(1+t)}=\log\frac t{1+t}+C$。

**ねらい** 分母の $xe^x$ を 1 つの文字と見て、分子の $x+1$ が $(xe^x)'$ の一部だと気づくか。足りない $e^x$ を分母分子にかけて補う。

## 問題9

**答え** $\dfrac1{2\sqrt2}\log\dfrac{x^2-\sqrt2x+1}{x^2+\sqrt2x+1}+C$

**骨子**
1. $x\ne0$ で分母分子を $x^2$ で割ると $\frac{1-\frac1{x^2}}{x^2+\frac1{x^2}}$。$t=x+\frac1x$ とおくと $dt=\left(1-\frac1{x^2}\right)dx$、$x^2+\frac1{x^2}=t^2-2$。
2. $\int\frac{dt}{t^2-2}=\frac1{2\sqrt2}\log\left|\frac{t-\sqrt2}{t+\sqrt2}\right|$。$t=x+\frac1x$ を戻すと $\frac{t-\sqrt2}{t+\sqrt2}=\frac{x^2-\sqrt2x+1}{x^2+\sqrt2x+1}$(この値は常に正)。
3. $x=0$ を含む区間でも、答えの式を微分すると被積分関数に一致するので、すべての実数で成り立つ。

**ねらい** 分母 $x^4+1$ を因数分解して部分分数にすると計算が重い。**分子 $x^2-1$ を見て $x+\frac1x$ の微分を作る**と一気に片づく。$x^2+1$ が分子なら $x-\frac1x$ を使い、こちらは $\tan^{-1}$ が必要になる(高校の範囲外)。

## 問題10

**答え** $\dfrac{\sin x-x\cos x}{x\sin x+\cos x}+C$

**骨子**
1. $(x\sin x+\cos x)'=x\cos x$ に注目し、$\frac{x^2}{(x\sin x+\cos x)^2}=\frac x{\cos x}\cdot\frac{x\cos x}{(x\sin x+\cos x)^2}$ と分ける。
2. $\frac{x\cos x}{(x\sin x+\cos x)^2}$ の原始関数は $-\frac1{x\sin x+\cos x}$。$\left(\frac x{\cos x}\right)'=\frac{\cos x+x\sin x}{\cos^2x}$。部分積分すると $-\frac x{\cos x(x\sin x+\cos x)}+\int\frac{dx}{\cos^2x}$。
3. $\tan x-\frac x{\cos x(x\sin x+\cos x)}$ を通分すると $\frac{\sin x-x\cos x}{x\sin x+\cos x}$。

**ねらい** 分母を微分すると $x\cos x$ が出ることに気づき、分子の $x^2$ を「$x\cos x$ とその残り $\frac x{\cos x}$」に割り振る。部分積分の分け方を自分で設計する問題。途中に $\cos x$ で割る式が出るが、最後の答えは $\cos x=0$ でも意味をもち、微分すれば正しいことが確かめられる。
