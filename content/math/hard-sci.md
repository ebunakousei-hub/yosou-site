+++
title = "難化版 理科（150分）"
level = "hard"
note = "東京大学 前期 数学 2027年度予想 ／ 難化版 ／ 理科（150分）"
order = 2
+++

## 第1問（難）

(1) $0\le x\le1$ のとき、次の不等式が成り立つことを示せ。$x-\dfrac{x^2}{2}\le\log(1+x)\le x-\dfrac{x^2}{2}+\dfrac{x^3}{3}$

(2) 正の整数 $n$ に対し $a_n=\displaystyle\prod_{k=1}^{n}\left(1+\frac{k}{n^2}\right)$ とおく。極限 $\displaystyle\lim_{n\to\infty}n\left(a_n-\sqrt e\right)$ を求めよ。

## 第2問（難）

$n$ を 3 以上の整数とする。1 から $n$ までの整数が 1 つずつ書かれた $n$ 枚のカードを、無作為に横一列に並べる。左から順にカードを見ていき、それまでに見たどのカードの数よりも大きい数のカードが現れたとき「更新が起きた」ということにする(1 枚目では必ず更新が起きたものとする)。更新がちょうど $k$ 回起きる確率を $p_n(k)$ とする。

(1) $p_n(2)$ を求めよ。

(2) $p_n(3)$ を求めよ。ただし、答えは和の記号 $\sum$ を用いて表してよい。

(3) $p_n(3)>p_n(2)$ となる最小の $n$ を求めよ。

## 第3問（難）

四面体 OABC において、OA $=$ OB $=$ OC $=1$、$\angle\mathrm{AOB}=\angle\mathrm{BOC}=\angle\mathrm{COA}=\theta$ とする($0<\theta<\frac{2\pi}3$)。線分 OA 上の点 P と線分 BC 上の点 Q の距離 PQ の最小値を $d(\theta)$ とする。

(1) $d(\theta)$ を $\theta$ で表せ。

(2) $\theta$ が $0<\theta<\frac{2\pi}3$ の範囲を動くとき、$d(\theta)$ の最大値と、そのときの $\cos\theta$ の値を求めよ。

## 第4問（やや難）

(1) 連続する 2 つの正の整数の積は、平方数でないことを示せ。

(2) 連続する 3 つの正の整数の積は、平方数でないことを示せ。

(3) 連続する 4 つの正の整数の積は、平方数でないことを示せ。

## 第5問（難+）

複素数 $\alpha,\beta,\gamma$ が $|\alpha|=|\beta|=|\gamma|=1$ かつ $\alpha\beta\gamma=1$ を満たして動く。$w=\alpha+\beta+\gamma$ とおく。

(1) $\gamma=\cos\varphi+i\sin\varphi$ を固定したとき、$w$ が動く範囲を求めよ。

(2) $w$ が動く範囲を複素数平面上に図示し、その面積を求めよ。

## 第6問（難）

原点 O を中心とする半径 1 の球面上に 4 点 A, B, C, D がある。四面体 ABCD の体積の最大値を求めよ。
@@解答@@

## 第1問

**答え** $\dfrac{\sqrt e}{3}$

**骨子**
1. (1) 差をとって微分する($g'(x)=\frac{x^2}{1+x}$ など)。
2. (2) $\log a_n$ を上下からはさむ:
   - $\sum\frac k{n^2}=\frac12+\frac1{2n}$
   - $\sum\frac{k^2}{2n^4}=\frac{(n+1)(2n+1)}{12n^3}=\frac1{6n}+O\!\left(\frac1{n^2}\right)$
   - $\sum\frac{k^3}{3n^6}=\frac{(n+1)^2}{12n^4}=O\!\left(\frac1{n^2}\right)$

   よって $n\left(\log a_n-\frac12\right)\to\frac12-\frac16=\frac13$。
3. $n(a_n-\sqrt e)=\sqrt e\cdot n(e^{h_n}-1)$($h_n=\log a_n-\frac12$)と書き、$\frac{e^h-1}h\to1$ を使う。

## 第2問

**答え** (1) $p_n(2)=\dfrac1n\displaystyle\sum_{i=1}^{n-1}\frac1i$ (2) $p_n(3)=\dfrac1n\displaystyle\sum_{1\le i<j\le n-1}\frac1{ij}$ (3) $n=8$

**骨子**
1. **最後のカードに注目する**(オ): 最後のカードで更新が起きる ⇔ それが $n$(確率 $\frac1n$)。最初の $n-1$ 枚の大小の並び方は一様に分布し、そこでの更新の回数は最後のカードに影響されない。よって $np_n(k)=p_{n-1}(k-1)+(n-1)p_{n-1}(k)$。
2. $u_n=np_n(k)$ とおくと $u_n-u_{n-1}=p_{n-1}(k-1)$ となり、和をとるだけで解ける。$k=2$ では $p_m(1)=\frac1m$ を足し、$k=3$ では (1) の結果を足す。
3. (3) $p_n(3)>p_n(2)$ ⇔ $\sum_{i<j}\frac1{ij}>\sum_i\frac1i$。分数で計算すると $n=7$ では $\frac{29}{90}<\frac7{20}$ で不成立、$n=8$ では $\frac{469}{1440}>\frac{363}{1120}$ でわずかに成立する($0.32569$ と $0.32411$)。$n\le6$ も表を作って確かめる。

## 第3問

**答え** (1) $0<\theta\le\frac\pi2$ のとき $d(\theta)=\sqrt{\dfrac{1+\cos\theta}2-\cos^2\theta}$、$\frac\pi2<\theta<\frac{2\pi}3$ のとき $d(\theta)=\sqrt{\dfrac{1+\cos\theta}2}$ (2) 最大値 $\dfrac34$($\cos\theta=\frac14$ のとき)

**骨子**
1. $\vec a,\vec b,\vec c$ を O から A, B, C へのベクトルとし、$c=\cos\theta$ とおく。$|\vec a|=|\vec b|=|\vec c|=1$、互いの内積はすべて $c$。$\mathrm P=s\vec a$、$\mathrm Q=\vec b+t(\vec c-\vec b)$($0\le s,t\le1$)。
2. **Q を固定しない工夫**: $s$ を固定すると、$\mathrm{PQ}^2$ は $t$ の 2 次関数で、$\vec b$ と $\vec c$ を入れかえる対称性から $t=\frac12$(BC の中点 M)で最小。
3. $|s\vec a-\overrightarrow{\mathrm{OM}}|^2=s^2-2cs+\frac{1+c}2$($\vec a\cdot\overrightarrow{\mathrm{OM}}=c$、$|\overrightarrow{\mathrm{OM}}|^2=\frac{1+c}2$)。$0\le s\le1$ で最小になるのは、$c\ge0$ なら $s=c$、$c<0$ なら $s=0$(P$=$O)。
4. (2) $c\ge0$ では $\frac{1+c}2-c^2$ が $c=\frac14$ で最大 $\frac9{16}$。$c<0$ では $\frac{1+c}2<\frac12<\frac9{16}$。

**ねらい** 空間ベクトルで、ねじれの位置にある 2 本の**線分**(直線ではない)の最短距離を求める。共通垂線の足が線分からはみ出すと最短の点が端 O に移る、という場合分け(吟味)と、対称性で Q を中点に固定する発想が山場。正四面体($\theta=\frac\pi3$)では $\frac1{\sqrt2}$ になる(検算)。

## 第4問

**答え** 略(いずれも証明問題)

**骨子**
1. (1) $n^2<n(n+1)<(n+1)^2$。
2. (2) $n(n+1)(n+2)=(n+1)\{(n+1)^2-1\}$。$n+1$ と $(n+1)^2-1$ は互いに素なので、積が平方数ならどちらも平方数。ところが $n^2<(n+1)^2-1<(n+1)^2$。
3. (3) $n(n+1)(n+2)(n+3)=(n^2+3n)(n^2+3n+2)=(n^2+3n+1)^2-1$ で、これは $(n^2+3n)^2$ と $(n^2+3n+1)^2$ の間にある。

**ねらい** 特別な定理を使わず、「互いに素な因数に分ける」「隣り合う平方数ではさむ」という発想だけで解く整数問題。(3) は 4 つを 2 つずつ組にすると $(\text{平方})-1$ の形が現れることに気づけるかが山場。

## 第5問

**答え** (1) 中点が $\gamma$、向きが $\gamma^{-1/2}$ 方向(偏角 $-\varphi/2$)の、長さ 4 の線分(両端を含む)。
(2) 曲線 $z(\theta)=2e^{i\theta}+e^{-2i\theta}$(3つの尖った点をもつ曲線)と、その内部。面積は $2\pi$。

**骨子**
1. (1) $\alpha\beta=\gamma^{-1}=e^{-i\varphi}$ なので $\alpha,\beta=e^{i(-\varphi/2\pm\psi)}$ とおける。すると $w=e^{i\varphi}+2\cos\psi\,e^{-i\varphi/2}$。
2. (2) 線分の端点 $e^{i\varphi}+2e^{-i\varphi/2}$ は、$\theta=-\varphi/2$ とおくと $z(\theta)$ になる。範囲は、これらの線分(長さ一定の4)の和集合。
3. 逆像法: $w$ が範囲に入る ⇔ $t=we^{-i\theta}-e^{-3i\theta}$ が実数で $|t|\le2$ となる $\theta$ がある。境界はこれらの直線の包絡線で、$z(\theta)$ になる(ア・エ)。
4. 面積は $x=2\cos\theta+\cos2\theta,\ y=2\sin\theta-\sin2\theta$ として、$\frac12\int_0^{2\pi}(xy'-yx')d\theta=\frac12\int_0^{2\pi}(2-2\cos3\theta)d\theta=2\pi$。

## 第6問

**答え** $\dfrac{8\sqrt3}{27}$(正四面体のとき)

**骨子**
1. 平面 ABC と O の距離を $h\ (0\le h<1)$ とする。A, B, C は平面 ABC 上の半径 $\sqrt{1-h^2}$ の円周上にある。
2. 半径 $\rho$ の円に内接する三角形の面積は、正三角形のとき最大で $\frac{3\sqrt3}4\rho^2$。中心から 3 辺を見込む角を $2\theta_1,2\theta_2,2\theta_3$($\theta_1+\theta_2+\theta_3=\pi$)とすると面積は $\frac{\rho^2}2(\sin2\theta_1+\sin2\theta_2+\sin2\theta_3)$ で、2 つずつ和積の公式でまとめると最大が分かる(中心が三角形の外にある場合も、符号つきの角で同じ式が使える)。
3. D と平面 ABC の距離は、O をはさんで反対側の $1+h$ 以下。
4. よって $V\le\frac13\cdot\frac{3\sqrt3}4(1-h^2)(1+h)=\frac{\sqrt3}4(1+h)^2(1-h)$。微分すると $h=\frac13$ で最大で、$\frac{8\sqrt3}{27}$。等号は正四面体のとき。

**ねらい** 一問完結型。「平面 ABC を固定 → 底面の三角形を最大にする → 高さを最大にする → 平面の位置を動かす」と、固定する順を自分で決める。円に内接する三角形の面積の最大を、きちんと証明できるかで差がつく。
