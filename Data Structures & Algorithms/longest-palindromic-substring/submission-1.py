class Solution:
    def longestPalindrome(self, s: str) -> str:
        if not s:
            return ""

        # Step 1: Preprocess string to handle even & odd uniformly
        # e.g., "babad" -> "^#b#a#b#a#d#$"
        transformed = "^#" + "#".join(s) + "#$"
        n = len(transformed)
        
        # radius[i] stores the radius of the palindrome centered at i
        radius = [0] * n
        center = 0
        right = 0

        # Step 2: Traverse the transformed string
        for i in range(1, n - 1):
            i_mirror = 2 * center - i

            # Use the mirrored value if within the current boundary
            if i < right:
                radius[i] = min(right - i, radius[i_mirror])

            # Expand outward beyond the pre-computed radius
            while transformed[i + 1 + radius[i]] == transformed[i - 1 - radius[i]]:
                radius[i] += 1

            # If the expanded palindrome extends past 'right', adjust center and right
            if i + radius[i] > right:
                center = i
                right = i + radius[i]

        # Step 3: Find the maximum radius and map back to original indices
        max_rad = max(radius)
        center_index = radius.index(max_rad)
        
        start = (center_index - max_rad) // 2
        return s[start : start + max_rad]