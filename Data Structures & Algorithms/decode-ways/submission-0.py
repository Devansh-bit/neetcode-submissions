class Solution:

    def numDecodings(self, s: str) -> int:

        # A string starting with '0' cannot be decoded

        if not s or s[0] == '0':

            return 0



        # prev2 = ways to decode up to (i - 2)

        # prev1 = ways to decode up to (i - 1)

        prev2 = 1  # Base case for empty prefix

        prev1 = 1  # Base case for s[0] (since s[0] != '0')



        for i in range(1, len(s)):

            current = 0



            # 1-digit jump: valid if s[i] is '1' through '9'

            if s[i] != '0':

                current += prev1



            # 2-digit jump: valid if '10' <= s[i-1:i+1] <= '26'

            two_digit = int(s[i - 1 : i + 1])

            if 10 <= two_digit <= 26:

                current += prev2



            # If current == 0, we've hit an impossible decode (e.g. "30" or "00")

            if current == 0:

                return 0



            # Slide window forward

            prev2 = prev1

            prev1 = current



        return prev1