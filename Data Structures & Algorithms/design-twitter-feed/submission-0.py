from collections import defaultdict
from typing import List

class Twitter:

    def __init__(self):
        self.timestamp = 0
        self.following: dict[int, set[int]] = defaultdict(set)
        # userId -> list of (timestamp, tweetId)
        self.user_tweets: dict[int, list[tuple[int, int]]] = defaultdict(list)

    def postTweet(self, userId: int, tweetId: int) -> None:
        self.timestamp += 1
        self.user_tweets[userId].append((self.timestamp, tweetId))

    def getNewsFeed(self, userId: int) -> List[int]:
        # A user always sees their own tweets plus their followees' tweets
        feed_users = self.following[userId] | {userId}
        
        # Collect recent tweets from relevant users only (at most 10 per user)
        candidates = []
        for u in feed_users:
            candidates.extend(self.user_tweets[u][-10:])
        
        # Sort by timestamp descending and take the top 10 tweet IDs
        candidates.sort(key=lambda x: x[0], reverse=True)
        return [tweet_id for _, tweet_id in candidates[:10]]

    def follow(self, followerId: int, followeeId: int) -> None:
        if followerId != followeeId:
            self.following[followerId].add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        self.following[followerId].discard(followeeId)