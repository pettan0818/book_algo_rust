#include <iostream>
#include <string>
using namespace std;

int main()
{
    int N;
    cin >> N;

    int count = 0;
    for (int i = 0; i < N; ++i)
    {
        ++count;
        printf("%d\n", count);
    }
}