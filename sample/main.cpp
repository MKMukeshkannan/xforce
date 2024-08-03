#include <bits/stdc++.h>
using namespace std;

#ifndef ONLINE_JUDGE
#include "template.cpp"
#else
#define debug(...)
#define debugArr(...)
#endif

#define fast_io ios_base::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);
#define io freopen("input.txt", "r", stdin); freopen("output.txt", "w", stdout); freopen("debug.txt", "w", stderr);
#define endl '\n'

#define ll long long
#define ld long double
#define all(a) (a).begin(), (a).end()
#define hai cout<<"YES"<<endl;
#define iie cout<<"NO"<<endl;

void solve() {
  int n; cin >> n;
	for (int i = 0; i < n; i++) {
    cout << i + 1 << endl;
  };
};

int main() {
	fast_io;
	#ifndef ONLINE_JUDGE
		io; auto _clock_start = chrono::high_resolution_clock::now();
	#endif

	int t = 1;
	for (int i = 0; i < t; i++) {
	#ifndef ONLINE_JUDGE
		cout<<"CASE "<<i+1<<":\n";
	#endif
		solve();
	}
	#ifndef ONLINE_JUDGE
		cout<<"---------------------\nExecuted in "<<chrono::duration_cast<chrono::milliseconds>(chrono::high_resolution_clock::now()- _clock_start).count()<<"ms."<<endl;
	#endif

	return 0;
};
