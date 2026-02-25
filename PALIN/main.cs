// wygenerowane przez Copilot

using System;

public class Program
{
    public static void Main(string[] args)
    {
        // Odczyt liczby przypadków testowych
        int t = int.Parse(Console.ReadLine());
        for (int i = 0; i < t; i++)
        {
            // Wczytanie liczby jako ciąg znaków (bez zer wiodących)
            string K = Console.ReadLine().Trim();
            string nextPalindrome = GetNextPalindrome(K);
            Console.WriteLine(nextPalindrome);
        }
    }

    /// <summary>
    /// Zwraca najmniejszy palindrom większy od K.
    /// </summary>
    /// <param name="K">Liczba wejściowa jako string.</param>
    /// <returns>Najmniejszy palindrom większy od K.</returns>
    public static string GetNextPalindrome(string K)
    {
        // Jeśli liczba składa się wyłącznie z dziewiątek, zwracamy postać 1,0...,1
        if (IsAll9s(K))
            return "1" + new string('0', K.Length - 1) + "1";

        int n = K.Length;
        string left, center = "", candidate;

        // Dla liczb o parzystej długości
        if (n % 2 == 0)
        {
            left = K.Substring(0, n / 2);
            candidate = left + Reverse(left);
            // Jeśli wygenerowany palindrom jest większy, zwróć go
            if (string.Compare(candidate, K) > 0)
                return candidate;
            else
            {
                // Inkrementujemy lewą część
                string newLeft = IncrementString(left);
                // Jeżeli nastąpiło rozszerzenie długości (np. z "99" na "100")
                if (newLeft.Length > left.Length)
                    return "1" + new string('0', n - 1) + "1";
                return newLeft + Reverse(newLeft);
            }
        }
        // Dla liczb o nieparzystej długości
        else
        {
            left = K.Substring(0, n / 2);
            center = K.Substring(n / 2, 1);
            candidate = left + center + Reverse(left);
            if (string.Compare(candidate, K) > 0)
                return candidate;
            else
            {
                // Łączymy lewą część ze środkową jako "prefix"
                string prefix = left + center;
                string newPrefix = IncrementString(prefix);
                if (newPrefix.Length > prefix.Length)
                    return "1" + new string('0', n - 1) + "1";
                // Nowy podział na lewą część i środek
                string newLeft = newPrefix.Substring(0, newPrefix.Length - 1);
                string newCenter = newPrefix.Substring(newPrefix.Length - 1, 1);
                return newLeft + newCenter + Reverse(newLeft);
            }
        }
    }

    /// <summary>
    /// Odwraca przekazany ciąg znaków.
    /// </summary>
    public static string Reverse(string s)
    {
        char[] arr = s.ToCharArray();
        Array.Reverse(arr);
        return new string(arr);
    }

    /// <summary>
    /// Zwraca true, jeśli wszystkie znaki w s to '9'.
    /// </summary>
    public static bool IsAll9s(string s)
    {
        foreach (char c in s)
        {
            if (c != '9')
                return false;
        }
        return true;
    }

    /// <summary>
    /// Inkrementuje liczbę reprezentowaną przez string, np. "129" -> "130".
    /// </summary>
    public static string IncrementString(string s)
    {
        char[] digits = s.ToCharArray();
        int i = digits.Length - 1;
        while (i >= 0)
        {
            if (digits[i] == '9')
            {
                digits[i] = '0';
                i--;
            }
            else
            {
                digits[i] = (char)(digits[i] + 1);
                break;
            }
        }
        if (i < 0)
        {
            // W przypadku rozszerzenia długości (np. "99" -> "100")
            return "1" + new string(digits);
        }
        return new string(digits);
    }
}
