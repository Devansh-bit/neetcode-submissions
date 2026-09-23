class Solution:
    def numDecodings(self, s: str) -> int:
        if not s or s[0] == '0':
            return 0
        prev = 1
        prev_prev = 1
        for i in range(1, len(s)):
            current = 0
            if s[i] != '0':
                current += prev
            
            two_digit = int(s[i - 1 : i + 1])
            if 10 <= two_digit <= 26:
                current += prev_prev

            if current == 0:
                return 0
            
            prev, prev_prev = current, prev
        return prev