+++
title = "難化年② 理科（150分）"
level = "hard"
note = "東京大学 前期 数学 予想問題 ／ 難化年②（軸: 整数と数え上げ） ／ 理科（150分）"
order = 10
+++

## 第1問（難）

$x>0$ に対し $F(x)=\displaystyle\int_x^{2x}\frac{\sin t}{t}\,dt$ とおく。$F(x)$ が最大値をとる $x$ と、最小値をとる $x$ を求めよ。

## 第2問（難）

1 から 9 までの数字が 1 つずつ書かれた 9 枚のカードから 3 枚を順に取り出し、取り出した順に百の位・十の位・一の位として並べて 3 桁の整数 $N$ を作る。

(1) $N$ が 3 の倍数となる確率を求めよ。

(2) $N$ が 11 の倍数となる確率を求めよ。

(3) $N$ が 37 の倍数となる確率を求めよ。

(4) $N$ が 3 の倍数であるとき、$N$ が 11 の倍数でもある条件付き確率を求めよ。

## 第3問（やや難）

座標空間内に、平面 $\alpha: z=0$ と平面 $\beta: z=\sqrt3\,y$ がある。点 $\mathrm P_0(1,1,1)$ から始めて、点 $\mathrm P_1,\mathrm P_2,\dots$ を次のように定める。
- $n$ が偶数のとき、$\mathrm P_{n+1}$ は $\mathrm P_n$ から平面 $\alpha$ に下ろした垂線の足
- $n$ が奇数のとき、$\mathrm P_{n+1}$ は $\mathrm P_n$ から平面 $\beta$ に下ろした垂線の足

(1) $\mathrm P_1,\mathrm P_2,\mathrm P_3$ の座標を求めよ。

(2) $\mathrm P_n$ の座標を $n$ の式で表し、$n\to\infty$ のとき $\mathrm P_n$ が近づく点を求めよ。

(3) 折れ線 $\mathrm P_0\mathrm P_1\mathrm P_2\cdots$ の長さ $\displaystyle\sum_{n=0}^{\infty}\mathrm P_n\mathrm P_{n+1}$ を求めよ。

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

**答え** 最大値をとるのは $x=\dfrac\pi3$、最小値をとるのは $x=\pi$
**骨子**
1. $F'(x)=\frac{\sin2x-\sin x}x=\frac{\sin x(2\cos x-1)}x$。極大は $x=\frac\pi3+2k\pi,\ \frac{5\pi}3+2k\pi$、極小は $x=k\pi$($k\ge1$)。符号は $(0,\frac\pi3)$ で正、$(\frac\pi3,\pi)$ で負、$(\pi,\frac{5\pi}3)$ で正、$(\frac{5\pi}3,2\pi)$ で負、…と周期 $2\pi$ でくり返す。
2. 遠くの極値をまとめて抑える評価を自分で用意する。部分積分 $F(x)=\left[-\frac{\cos t}t\right]_x^{2x}-\int_x^{2x}\frac{\cos t}{t^2}dt$ から $|F(x)|\le\frac1{2x}+\frac1x+\left(\frac1x-\frac1{2x}\right)=\frac2x$。
3. 最大: $\left[\frac\pi3,\frac{2\pi}3\right]$ で $\sin t\ge\frac{\sqrt3}2$ なので $F\left(\frac\pi3\right)\ge\frac{\sqrt3}2\log2>0.6$。ほかの極大点は $x\ge\frac{5\pi}3$ で、$|F|\le\frac6{5\pi}<0.39$。
4. 最小: $(\pi,2\pi)$ で $\sin t<0$、$\frac1t>\frac1{2\pi}$ なので $F(\pi)<\frac1{2\pi}\int_\pi^{2\pi}\sin t\,dt=-\frac1\pi$。ほかの極小点は $x\ge2\pi$ で $F\ge-\frac2x\ge-\frac1\pi$。

**ねらい** 一問完結型。極値点は無限にあるので、「遠くの極値はまとめて評価で抑え、近くの候補だけ比べる」という方針を自分で立てる必要がある。最小値の比較では評価がちょうど $-\frac1\pi$ で並ぶので、**不等号が真に成り立つ側**を使う吟味が要る。

## 第2問

**答え** (1) $\dfrac5{14}$ (2) $\dfrac19$ (3) $\dfrac1{42}$ (4) $\dfrac{11}{90}$
**骨子**
1. 全部で $9\cdot8\cdot7=504$ 通りで、どれも同様に確からしい。百・十・一の位を $a,b,c$ とする。
2. (1) 3 の倍数 ⇔ $a+b+c$ が 3 の倍数。数字の組だけで決まるので、組 ${}_9\mathrm C_3=84$ 通りのうち、3 で割った余りが「3 つとも同じ」3 組と「全部違う」$3^3=27$ 組の計 30 組。$\frac{30}{84}$。
3. (2) 11 の倍数 ⇔ $a-b+c$ が 11 の倍数 ⇔ $b=a+c$ または $b=a+c-11$。前者は $a\ne c$、$a+c\le9$ の順序つきの組で 32 通り、後者は $a+c\ge12$ の組で 24 通り。計 56 通り。
4. (3) $1000=27\cdot37+1$ なので、$N=100a+10b+c$ が 37 の倍数なら、位をずらした $100b+10c+a=10N-999a$ も 37 の倍数。37 の倍数の 3 桁の整数(25 個)のうち、数字が相異なり 0 を含まないのは $148,185,259$ とそのずらし(各 3 個)、および $296,481,592$ に対応する組で、計 12 個。$\frac{12}{504}$。
5. (4) 3 の倍数は $504\cdot\frac5{14}=180$ 通り。そのうち 11 の倍数: $b=a+c$ の場合は和が $2b$ なので $b\in\{3,6,9\}$ で $2+4+8=14$ 通り。$b=a+c-11$ の場合は和が $2b+11$ なので $b\equiv2\pmod3$、$b=2$ で 6 通り、$b=5$ で 2 通り。計 22 通りで、$\frac{22}{180}$。

**ねらい** 素朴な設定から、倍数の判定法(3・11・37)を自分で選んで数え上げる。37 では「位をずらしても倍数のまま」という構造に気づくと、調べる数が 3 分の 1 になる。(4) は 2 つの条件を同時に満たす場合を、(2) の場合分けにもう 1 つの条件を重ねて数える。

## 第3問

**答え** (1) $\mathrm P_1(1,1,0)$、$\mathrm P_2\left(1,\frac14,\frac{\sqrt3}4\right)$、$\mathrm P_3\left(1,\frac14,0\right)$ (2) $k\ge1$ について $\mathrm P_{2k-1}\left(1,\left(\frac14\right)^{k-1},0\right)$、$\mathrm P_{2k}\left(1,\frac14\left(\frac14\right)^{k-1},\frac{\sqrt3}4\left(\frac14\right)^{k-1}\right)$。近づく点は $(1,0,0)$ (3) $1+\sqrt3$
**骨子**
1. 2 平面はどちらも $x$ 軸を含むので、$x$ 座標は 1 のまま。$yz$ 平面で見ると、直線 $z=0$ と直線 $z=\sqrt3y$(なす角 $\frac\pi3$)への正射影をくり返すことになる。
2. 平面 $\beta$ の単位法線ベクトルは $\vec n=\left(0,-\frac{\sqrt3}2,\frac12\right)$ で、$\beta$ への正射影は $\vec p\mapsto\vec p-(\vec p\cdot\vec n)\vec n$。
3. 正射影のたびに、$x$ 軸からの距離 $r_n$ は $\cos\frac\pi3=\frac12$ 倍になり、移動距離は $r_n\sin\frac\pi3$。最初の 1 回($\mathrm P_0\to\mathrm P_1$)だけは長さ 1。
4. (3) $1+\sum_{n\ge1}\frac{\sqrt3}2\left(\frac12\right)^{n-1}=1+\sqrt3$。

**ねらい** 空間ベクトル(正射影)と数列(等比数列・無限級数)の融合。2 平面が共通の直線を含むことに気づくと、空間の問題が平面の問題に落ちる。

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
