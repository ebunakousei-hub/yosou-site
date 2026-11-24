+++
title = "難化年② 理科（150分）"
level = "hard"
note = "東京大学 前期 数学 予想問題 ／ 難化年②（軸: 整数と数え上げ） ／ 理科（150分）"
order = 10
+++

## 第1問（難）

$x>0$ に対し $F(x)=\displaystyle\int_x^{2x}\frac{\sin t}{t}\,dt$ とおく。

(1) $F(x)$ が極大値をとる $x$ と、極小値をとる $x$ をすべて求めよ。

(2) $x>0$ のとき $|F(x)|\le\dfrac2x$ を示せ。

(3) $F(x)$ は $x=\dfrac\pi3$ で最大値を、$x=\pi$ で最小値をとることを示せ。

## 第2問（難）

袋 A には赤玉 3 個が、袋 B には白玉 3 個が入っている。「A と B からそれぞれ玉を 1 個ずつ無作為に取り出し、A から取り出した玉を B に、B から取り出した玉を A に入れる」という操作をくり返す。$n$ 回の操作のあとに A に入っている赤玉の個数を $X_n$ とする。

(1) $X_n$ の期待値 $E(X_n)$ を求めよ。

(2) $X_n=0$ または $X_n=3$ となる確率を求めよ。

(3) $X_n=3$ となる確率を求めよ。

## 第3問（やや難）

1 辺の長さが 1 の正四面体 ABCD があり、辺 AB の中点を M、辺 CD の中点を N とする。

(1) 直線 MN に垂直な平面による正四面体の切り口(空でないもの)は、長方形または線分であることを示せ。

(2) 正四面体を直線 MN のまわりに 1 回転させてできる立体の体積を求めよ。

## 第4問（難）

正の整数 $n$ に対し、$xy\le n$ を満たす正の整数の組 $(x,y)$ の個数を $T(n)$ とし、正の整数 $k$ の正の約数の個数を $d(k)$ とする。

(1) $T(n)=d(1)+d(2)+\cdots+d(n)$ を示せ。

(2) $T(n)$ が奇数であることは、$\sqrt n$ 以下の最大の整数が奇数であることと同値であることを示せ。

(3) $1\le n\le2027$ を満たす整数 $n$ のうち、$T(n)$ が奇数となるものの個数を求めよ。

(4) 極限 $\displaystyle\lim_{n\to\infty}\frac{T(n)}{n\log n}$ を求めよ。

## 第5問（難）

$n$ を 3 以上の整数とし、$\omega_k=\cos\dfrac{2k\pi}n+i\sin\dfrac{2k\pi}n\ (k=0,1,\dots,n-1)$ とする。複素数 $z$ が $|z|=1$ を満たして動く。

(1) $\displaystyle\prod_{k=0}^{n-1}|z-\omega_k|$ の最大値を求めよ。

(2) $\displaystyle\sum_{k=0}^{n-1}|z-\omega_k|^2$ と $\displaystyle\sum_{k=0}^{n-1}|z-\omega_k|^4$ は $z$ によらない定数であることを示し、その値を求めよ。

(3) $\displaystyle\sum_{k=0}^{n-1}|z-\omega_k|$ の最大値と最小値を求めよ。

## 第6問（難）

2 以上の整数 $n$ に対し、$f_n(x)=(x+1)^n-x^n-1$ とおく。

(1) $f_n(x)$ が $x^2+x+1$ で割り切れるための $n$ の条件を求めよ。

(2) $f_n(x)$ が $(x^2+x+1)^2$ で割り切れるための $n$ の条件を求めよ。

(3) $f_n(x)$ の係数がすべて $n$ の倍数であることは、$n$ が素数であることと同値であることを示せ。

@@解答@@

## 第1問

**答え** (1) 極大: $x=\frac\pi3+2k\pi,\ \frac{5\pi}3+2k\pi\ (k=0,1,2,\dots)$、極小: $x=k\pi\ (k=1,2,3,\dots)$ (2)(3) 略
**骨子**
1. (1) $F'(x)=\frac{\sin2x-\sin x}x=\frac{\sin x(2\cos x-1)}x$。符号は $(0,\frac\pi3)$ で正、$(\frac\pi3,\pi)$ で負、$(\pi,\frac{5\pi}3)$ で正、$(\frac{5\pi}3,2\pi)$ で負、…と周期 $2\pi$ でくり返す。
2. (2) 部分積分: $F(x)=\left[-\frac{\cos t}t\right]_x^{2x}-\int_x^{2x}\frac{\cos t}{t^2}dt$ から $|F(x)|\le\frac1{2x}+\frac1x+\left(\frac1x-\frac1{2x}\right)=\frac2x$。
3. (3) 最大: $\left[\frac\pi3,\frac{2\pi}3\right]$ で $\sin t\ge\frac{\sqrt3}2$ なので $F\left(\frac\pi3\right)\ge\frac{\sqrt3}2\log2>0.6$。ほかの極大点は $x\ge\frac{5\pi}3$ で、$|F|\le\frac6{5\pi}<0.39$。
4. 最小: $(\pi,2\pi)$ で $\sin t<0$、$\frac1t>\frac1{2\pi}$ なので $F(\pi)<\frac1{2\pi}\int_\pi^{2\pi}\sin t\,dt=-\frac1\pi$。ほかの極小点は $x\ge2\pi$ で $F\ge-\frac2x\ge-\frac1\pi$。

**ねらい** 値が求まらない積分の大小を、評価で決める。最小値の比較では評価がちょうど $-\frac1\pi$ で並ぶので、**不等号が真に成り立つ側**を使う吟味が要る。

## 第2問

**答え** (1) $\dfrac32+\dfrac32\left(\dfrac13\right)^n$ (2) $\dfrac1{10}+\dfrac9{10}\left(-\dfrac19\right)^n$ (3) $\dfrac1{20}+\dfrac{1+(-1)^n}4\left(\dfrac13\right)^n+\dfrac9{20}\left(-\dfrac19\right)^n$
**骨子**
1. A の赤玉が $k$ 個のとき、次に $k-1$ 個になる確率は $\frac{k^2}9$、$k+1$ 個になる確率は $\frac{(3-k)^2}9$。
2. (1) $E(X_{n+1})=E(X_n)+E\!\left[\frac{(3-X_n)^2-X_n^2}9\right]=\frac13E(X_n)+1$。
3. (2)(3) $a_n,b_n,c_n,d_n$ を $X_n=3,2,1,0$ の確率とすると、$a_{n+1}=\frac19b_n$、$d_{n+1}=\frac19c_n$。$s_n=a_n+d_n$ は $s_{n+1}=\frac19(1-s_n)$。$t_n=a_n-d_n$、$u_n=b_n-c_n$ は $t_{n+1}=\frac19u_n$、$u_{n+1}=t_n$(**$b,c$ から出る「とどまる」項が差では消える**)。よって $t_{n+2}=\frac19t_n$。$a_n=\frac{s_n+t_n}2$。

**ねらい** 期待値(数学Ａ)の漸化式と、4 つの状態の推移を「和と差」に分けて 2 つの漸化式にする構造の発見。差のほうは 2 回ごとにしか動かないので、$n$ の偶奇で式が変わる。

## 第3問

**答え** (2) $\dfrac{\sqrt2}{12}\pi$
**骨子**
1. (1) 正四面体は $(\pm1,\pm1,\pm1)$ の 4 点(符号の積が正)を $\frac1{2\sqrt2}$ 倍したものとしてよい。MN は座標軸の 1 本(長さ $\frac1{\sqrt2}$)で、AB, CD はそれと垂直な互いに直交する方向。軸に垂直な平面の切り口は、AB に平行な辺と CD に平行な辺をもつ長方形(辺の長さ $1-\sigma$ と $\sigma$、$0\le\sigma\le1$)。
2. 長方形の中心は軸上にあるので、回転した断面は半径 $\frac12\sqrt{\sigma^2+(1-\sigma)^2}$(対角線の半分)の円板。
3. $V=\pi\int_0^{1/\sqrt2}\frac{\sigma^2+(1-\sigma)^2}4\,dh$、$h=\frac\sigma{\sqrt2}$。

**ねらい** 空間ベクトルで切り口の形を決める。対称性のよい座標をとれるかどうかで計算量が大きく変わる。

## 第4問

**答え** (3) $993$ (4) $1$
**骨子**
1. (1) 文科第3問 (1) と同じ。
2. (2) 約数を $e$ と $\frac ke$ の組にすると、$d(k)$ が奇数 ⇔ $k$ が平方数。よって $T(n)$ の偶奇は $n$ 以下の平方数の個数 $\lfloor\sqrt n\rfloor$ の偶奇と一致する。
3. (3) 文科第3問 (3) と同じ。
4. (4) $T(n)=\sum_{x=1}^n\left\lfloor\frac nx\right\rfloor$ なので $n\sum_{x=1}^n\frac1x-n\le T(n)\le n\sum_{x=1}^n\frac1x$。$\log(n+1)\le\sum_{x=1}^n\frac1x\le1+\log n$ ではさむ。

**ねらい** 文科の問題に、和を積分で評価して極限を出す小問を加えた。$\sum\frac1x$ を $\log$ ではさむ定石が使えるか。

## 第5問

**答え** (1) 2 (2) $2n$、$6n$ (3) 最大値 $\dfrac2{\sin\frac\pi{2n}}$(弧の中点)、最小値 $\dfrac2{\tan\frac\pi{2n}}$(頂点)
**骨子**
1. (1) $\prod(z-\omega_k)=z^n-1$ なので $|z^n-1|\le2$、等号は $z^n=-1$。
2. (2) $|z-\omega_k|^2=2-2\,\mathrm{Re}(z\overline{\omega_k})$。$\sum\overline{\omega_k}=0$、$n\ge3$ なら $\sum\overline{\omega_k}^2=0$ を使う。4 乗は $\{\mathrm{Re}(\cdot)\}^2=\frac{1+\mathrm{Re}(\cdot)^2}2$ と書きかえる。
3. (3) $z=\cos\theta+i\sin\theta$ とすると $|z-\omega_k|=2\left|\sin\left(\frac\theta2-\frac{k\pi}n\right)\right|$。周期 $\frac\pi n$ なので $\frac\theta2=\psi\in[0,\frac\pi n]$ としてよく、このとき全部の $\sin$ が 0 以上。等間隔の角の $\sin$ の和の公式から $\sum=\frac{2\cos(\psi-\frac\pi{2n})}{\sin\frac\pi{2n}}$。

**ねらい** 1 の $n$ 乗根と三角関数の和の組み合わせ。(3) で絶対値を外すために「周期で 1 区間に縮める」発想が山場。

## 第6問

**答え** (1) $n\equiv1,5\pmod6$ (2) $n\equiv1\pmod6$ (3) 略
**骨子**
1. (1) $\omega$ を $x^2+x+1=0$ の解とすると $\omega+1=-\omega^2$。$f_n(\omega)=(-1)^n\omega^{2n}-\omega^n-1=0$ を $n$ の mod 6 で調べる。
2. (2) 重解の条件 $f_n'(\omega)=n\{(\omega+1)^{n-1}-\omega^{n-1}\}=0$ ⇔ $(-\omega^2)^{n-1}=\omega^{n-1}$。(1) と合わせて $n\equiv1\pmod6$。例: $f_7=7x(x+1)(x^2+x+1)^2$。
3. (3) 係数は ${}_n\mathrm C_k\ (1\le k\le n-1)$。$n$ が素数なら、$k\cdot{}_n\mathrm C_k=n\cdot{}_{n-1}\mathrm C_{k-1}$ と $n$, $k$ が互いに素であることから $n$ の倍数。$n$ が合成数で素因数 $p$ をもつとき、${}_n\mathrm C_p=\frac np\cdot{}_{n-1}\mathrm C_{p-1}$ で、${}_{n-1}\mathrm C_{p-1}$ の分子 $(n-1)\cdots(n-p+1)$ は $p$ の倍数を含まないので、${}_n\mathrm C_p$ は $n$ の倍数でない。

**ねらい** 整式の割り算(1 の3乗根での値)、素数、二項係数を 1 問に入れた。(2) で微分を使う発想(2023年度理5 と同じ)と、(3) で合成数の側の反例として $k=p$ を選ぶことが山場。
