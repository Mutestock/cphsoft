package dk.cphbusiness.mrv.twitterclone.usermanagement;

import dk.cphbusiness.mrv.twitterclone.TestBase;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class UnfollowUserIT extends TestBase {
    @Test
    public void mustBeAbleToUnfollowUser() {
        // Arrange
        var albert = getAlbert();
        var benny = getBenny();
        var carl = getCarl();
        userManagement.createUser(albert);
        userManagement.createUser(benny);
        userManagement.createUser(carl);
        userManagement.followUser(albert.getUsername(), benny.getUsername());
        userManagement.followUser(albert.getUsername(), carl.getUsername());
        assertTrue(
                userManagement.getFollowedUsers(albert.getUsername()).containsAll(List.of(benny.getUsername(), carl.getUsername())));

        // Act
        userManagement.unfollowUser(albert.getUsername(), carl.getUsername());

        // Assert
        var followedUsers = userManagement.getFollowedUsers(albert.getUsername());
        assertTrue(followedUsers.contains(benny.getUsername()));
        assertFalse(followedUsers.contains(carl.getUsername()));
    }

    @Test
    public void mustReturnFalseIfUserDoesNotExist() {
        // Arrange
        var albert = getAlbert();
        userManagement.createUser(albert);

        // Act
        var result = userManagement.unfollowUser("bleb", albert.getUsername());

        // Assert
        assertFalse(result);
    }
}
