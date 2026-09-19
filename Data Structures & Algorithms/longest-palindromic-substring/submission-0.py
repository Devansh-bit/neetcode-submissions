class Solution:
    def longestPalindrome(self, s: str) -> str:
        if not s:
            return ""

        # Track the start index and length of the longest palindrome found
        best_start = 0
        max_length = 0

        def expand_around_center(left: int, right: int) -> None:
            nonlocal best_start, max_length
            
            # Expand outward as long as pointers are valid and characters match
            while left >= 0 and right < len(s) and s[left] == s[right]:
                left -= 1
                right += 1

            # When the loop terminates, left and right have overstepped by 1.
            # Valid palindrome starts at (left + 1) and ends at (right - 1).
            current_len = (right - 1) - (left + 1) + 1  # simplifies to right - left - 1
            if current_len > max_length:
                max_length = current_len
                best_start = left + 1

        for i in range(len(s)):
            # Case 1: Odd-length palindrome (center at s[i])
            expand_around_center(i, i)
            
            # Case 2: Even-length palindrome (center between s[i] and s[i + 1])
            expand_around_center(i, i + 1)

        return s[best_start : best_start + max_length]