use std::{cmp::max, io::{self, Read}};

struct Scan{b:Vec<u8>,i:usize}
impl Scan{
 fn new()->Self{let mut s=String::new();io::stdin().read_to_string(&mut s).unwrap();Self{b:s.into_bytes(),i:0}}
 fn ws(&mut self){while self.i<self.b.len()&&self.b[self.i].is_ascii_whitespace(){self.i+=1;}}
 fn u(&mut self)->usize{self.ws();let mut x=0;while self.i<self.b.len()&&!self.b[self.i].is_ascii_whitespace(){x=x*10+(self.b[self.i]-b'0')as usize;self.i+=1;}x}
 fn i64(&mut self)->i64{self.ws();let mut x=0i64;while self.i<self.b.len()&&!self.b[self.i].is_ascii_whitespace(){x=x*10+(self.b[self.i]-b'0')as i64;self.i+=1;}x}
}

struct Dsu{p:Vec<usize>,s:Vec<usize>}
impl Dsu{
 fn new(n:usize)->Self{Self{p:(0..n).collect(),s:vec![1;n]}}
 fn find(&mut self,x:usize)->usize{if self.p[x]==x{x}else{let r=self.find(self.p[x]);self.p[x]=r;r}}
 fn unite(&mut self,a:usize,b:usize)->bool{let mut a=self.find(a);let mut b=self.find(b);if a==b{return false;}if self.s[a]<self.s[b]{std::mem::swap(&mut a,&mut b);}self.p[b]=a;self.s[a]+=self.s[b];true}
}

fn bottleneck(mut a:usize,mut b:usize,d:&[usize],up:&[Vec<usize>],mx:&[Vec<i64>])->i64{
 let mut ans=0;let lg=up.len();
 if d[a]<d[b]{std::mem::swap(&mut a,&mut b);}let diff=d[a]-d[b];
 for k in 0..lg{if (diff>>k)&1==1{ans=max(ans,mx[k][a]);a=up[k][a];}}
 if a==b{return ans;}
 for k in (0..lg).rev(){if up[k][a]!=up[k][b]{ans=max(ans,mx[k][a]);ans=max(ans,mx[k][b]);a=up[k][a];b=up[k][b];}}
 ans=max(ans,mx[0][a]);ans=max(ans,mx[0][b]);ans
}

fn main(){
 let mut sc=Scan::new();
 let n=sc.u();let m=sc.u();let q=sc.u();
 let mut e=Vec::with_capacity(m);
 for _ in 0..m{let u=sc.u()-1;let v=sc.u()-1;let w=sc.i64();e.push((w,u,v));}
 e.sort_unstable();
 let mut dsu=Dsu::new(n);
 let mut g=vec![Vec::<(usize,i64)>::new();n];
 for (w,u,v) in e{if dsu.unite(u,v){g[u].push((v,w));g[v].push((u,w));}}
 let mut lg=1;while (1usize<<lg)<=n{lg+=1;}
 let mut up=vec![vec![0;n];lg];
 let mut mx=vec![vec![0;n];lg];
 let mut dep=vec![0;n];
 let mut vis=vec![false;n];
 let mut st=vec![0usize];vis[0]=true;
 while let Some(u)=st.pop(){for &(v,w) in &g[u]{if !vis[v]{vis[v]=true;dep[v]=dep[u]+1;up[0][v]=u;mx[0][v]=w;st.push(v);}}}
 for k in 1..lg{for v in 0..n{let p=up[k-1][v];up[k][v]=up[k-1][p];mx[k][v]=max(mx[k-1][v],mx[k-1][p]);}}
 let mut out=String::new();
 for _ in 0..q{
 let a=sc.u()-1;let b=sc.u()-1;let f=sc.i64();let k=sc.i64();let l=sc.i64();
 let need=bottleneck(a,b,&dep,&up,&mx);
 let ans=if k==0{if l>=need{f}else{0}}
 else if l>=need{f}
 else{let first=(need-l+k-1)/k; if first>=f{0}else{f-first}};
 out.push_str(&format!("{}\n",ans));
 }
 print!("{}",out);
}
