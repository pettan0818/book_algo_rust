#include <iostream>
#include <string>
#include <vector>
#include <cmath>

using namespace std;

int get_specific_digit(int source, int digit_num)
{
    int floored = source / (int)pow(10.0, (float)(digit_num - 1));
    return floored % 10;
}

int get_len_int(int check_target)
{
    if (check_target == 0)
    {
        return 1;
    }
    return (int)log10((float)check_target) + 1;
}

bool check_first_digit(int check_target)
{
    //cout << 'len: ' << get_len_int(check_target) << endl;
    if (get_len_int(check_target) != 6) // 桁数チェック
    {
        return false;
    }
    //cout << 'passed' << endl;
    return true;
    if (get_specific_digit(check_target, 6) != 6)
    {
        return false;
    }
    if (get_specific_digit(check_target, 5) != 6)
    {
        return false;
    }
    return true;
}

bool check_second_digit(int check_target)
{
    if (get_len_int(check_target) != 6) // 桁数チェック
    {
        return false;
    }
    if (get_specific_digit(check_target, 6) != 6)
    {
        return false;
    }
    return true;
}

bool check_third_digit(int check_target)
{
    if (get_len_int(check_target) != 7) // 桁数チェック
    {
        return false;
    }
    if (get_specific_digit(check_target, 3) != 6)
    {
        return false;
    }
    if (get_specific_digit(check_target, 4) != 6)
    {
        return false;
    }
    if (get_specific_digit(check_target, 5) != 6)
    {
        return false;
    }
    return true;
}

bool check_forth_digit(int check_target)
{
    if (get_len_int(check_target) != 6) // 桁数チェック
    {
        return false;
    }
    if (get_specific_digit(check_target, 1) != 6)
    {
        return false;
    }
    if (get_specific_digit(check_target, 4) != 6)
    {
        return false;
    }
    return true;
}

bool check_final_res(int check_target)
{
    if (get_len_int(check_target) != 10) // 桁数チェック
    {
        return false;
    }
    if (get_specific_digit(check_target, 5) != 6)
    {
        return false;
    }
    if (get_specific_digit(check_target, 6) != 6)
    {
        return false;
    }
    return true;
}

int main(void)
{
    setlocale(LC_ALL, "");

    for (int multipler = 1000; multipler < 10000; multipler++)
    {
        for (int source = 100000; source < 1000000; source++)
        {
            int res = 0;
            // 1st.
            // multipler per digit x source...
            int digit_1 = get_specific_digit(multipler, 1);
            int tester_first = source * digit_1;
            //cout << source << '*' << digit_1 << '=' << tester_first << endl;
            if (check_first_digit(tester_first))
            {
                res += tester_first;
                //cout << "passed_1: " << source << '*' << multipler << endl;
            }
            else
            {
                continue;
            }
            // 2nd.
            int digit_2 = get_specific_digit(multipler, 2);
            int tester_second = source * digit_2;
            if (check_second_digit(tester_second))
            {
                res += tester_second * 10;
                //cout << "passed_2: " << source << '*' << multipler << endl;
            }
            else
            {
                continue;
            }

            // 3rd.
            int digit_3 = get_specific_digit(multipler, 3);
            int tester_third = source * digit_3;
            if (check_third_digit(tester_third))
            {
                res += tester_second * 100;
                cout << "passed_3: " << source << '*' << multipler << endl;
            }
            else
            {
                continue;
            }

            // 4th.
            int digit_4 = get_specific_digit(multipler, 4);
            int tester_forth = source * digit_4;
            if (check_forth_digit(tester_forth))
            {
                res += tester_second * 1000;
                cout << "passed_4: " << source << '*' << multipler << endl;
            }
            else
            {
                continue;
            }

            // check final res.
            if (check_final_res(multipler * source))
            {
                cout << multipler << '*' << source << endl;
                return 0;
            }
            else
            {
                continue;
            }
        }
        cout << multipler << endl;
    }
}
