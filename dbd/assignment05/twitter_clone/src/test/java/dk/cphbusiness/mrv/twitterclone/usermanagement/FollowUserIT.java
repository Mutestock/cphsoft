package dk.cphbusiness.mrv.twitterclone.usermanagement;

import dk.cphbusiness.mrv.twitterclone.TestBase;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

public class FollowUserIT extends TestBase {
    @Test
    @DisplayName("Must be able to follow user")
    public void mustBeAbleToFollowUser() {
        // Arrange
        var albert = getAlbert();
        var benny = getBenny();
        userManagement.createUser(albert);
        userManagement.createUser(benny);

        // Act
        var result = userManagement.followUser(albert.getUsername(), albert.getUsername());

        // Assert
        assertTrue(result);
    }

    @Test
    @DisplayName("Followed user must have a follower")
    public void followedUserMustHaveFollower() {
        // Arrange
        var albert = getAlbert();
        var benny = getBenny();
        userManagement.createUser(albert);
        userManagement.createUser(benny);

        // Act
        userManagement.followUser(albert.getUsername(), benny.getUsername());
        var followingBenny = userManagement.getUsersFollowing(benny.getUsername());

        // Assert
        assertTrue(followingBenny.contains(albert.getUsername()),
                "if a follows b, then the result of getUsersFollowing(b) should contain a");
    }

    @Test
    @DisplayName("FollowUser must return null if username does not exist")
    public void followUserMustReturnNullIfUsernameDoesNotExist() {
        // Arrange
        var benny = getBenny();
        userManagement.createUser(benny);

        // Act
        var result = userManagement.followUser("albert", benny.getUsername());

        // Assert
        assertFalse(result);
    }

    @Test
    @DisplayName("FollowUser must return null if followed user does not exist")
    public void followUserMustReturnNullIfFollowedUserDoesNotExist() {
        // Arrange
        var albert = getAlbert();
        userManagement.createUser(albert);

        // Act
        var result = userManagement.followUser(albert.getUsername(), "benny");

        // Assert
        assertFalse(result);
    }

    @Test
    @DisplayName("Followed user must appear in the list of followed users")
    public void followedUserMustAppearInListOfFollowedUsers() {
        // Arrange
        var albert = getAlbert();
        var benny = getBenny();
        var carl = getCarl();
        var dennis = getDennis();
        userManagement.createUser(albert);
        userManagement.createUser(benny);
        userManagement.createUser(carl);
        userManagement.createUser(dennis);

        // Act
        userManagement.followUser(albert.getUsername(), benny.getUsername());
        userManagement.followUser(albert.getUsername(), carl.getUsername());

        // Assert
        Set<String> usernames = userManagement.getFollowedUsers(albert.getUsername());
        assertTrue(usernames.containsAll(List.of(benny.getUsername(), carl.getUsername())),
                "If followUser(a,b) was called, the result of getFollowedUsers(a) should contain b");
        assertFalse(usernames.contains(dennis.getUsername()),
                "If followUser(a,b) was never called, the result of getFollowedUsers(a) must not contain b");
    }

    @Test
    @DisplayName("User overview must contain number of followed users")
    public void userOverviewMustContainNumberOfFollowedUsers() {
        // Arrange
        var albert = getAlbert();
        var benny = getBenny();
        userManagement.createUser(albert);
        userManagement.createUser(benny);

        // Act
        var userOverview = userManagement.getUserOverview(albert.getUsername());

        // Assert
        assertEquals(0, userOverview.getNumFollowing());

        // Act
        userManagement.followUser(albert.getUsername(), benny.getUsername());
        userOverview = userManagement.getUserOverview(albert.getUsername());

        // Assert
        assertEquals(0, userOverview.getNumFollowers(),
                "NumFollowers should not have changed. a->b, a has 0 followers and b has 1.");
        assertEquals(1, userOverview.getNumFollowing(), "NumFollowing should have changed. a->b, a is following 1 user.");
    }

    @Test
    @DisplayName("User overview must contain number of following users")
    public void userOverviewMustContainNumberOfFollowingUsers() {
        // Arrange
        var albert = getAlbert();
        var benny = getBenny();
        userManagement.createUser(albert);
        userManagement.createUser(benny);

        // Act
        var userOverview = userManagement.getUserOverview(albert.getUsername());

        // Assert
        assertEquals(0, userOverview.getNumFollowers());

        // Act
        userManagement.followUser(albert.getUsername(), benny.getUsername());
        userOverview = userManagement.getUserOverview(benny.getUsername());

        // Assert
        assertEquals(1, userOverview.getNumFollowers());
    }

    @Test
    @DisplayName("getUsersFollowing must return null if username does not exist")
    public void getUsersFollowingMustReturnNullIfUsernameDoesNotExist() {
        // Arrange
        var albert = getAlbert();
        userManagement.createUser(albert);

        // Act
        var result = userManagement.getUsersFollowing("bleb");

        // Assert
        assertNull(result);
    }

    @Test
    @DisplayName("getFollowedUsers must return null if username does not exist")
    public void getFollowedUsersMustReturnNullIfUsernameDoesNotExist() {
        // Arrange
        // Act
        var result = userManagement.getFollowedUsers("bleb");

        // Assert
        assertNull(result);
    }
}
