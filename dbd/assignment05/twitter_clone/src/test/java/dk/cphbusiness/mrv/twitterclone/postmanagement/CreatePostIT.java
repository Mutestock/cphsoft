package dk.cphbusiness.mrv.twitterclone.postmanagement;

import dk.cphbusiness.mrv.twitterclone.TestBase;
import dk.cphbusiness.mrv.twitterclone.dto.Post;
import dk.cphbusiness.mrv.twitterclone.dto.UserCreation;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import org.junit.jupiter.params.provider.ValueSource;

import static org.junit.jupiter.api.Assertions.*;

public class CreatePostIT extends TestBase {
    private UserCreation albert;

    private void createAlbert() {
        albert = getAlbert();
        userManagement.createUser(albert);
    }

    private String createPost(String username, long timestamp) {
        time.setCurrentTime(timestamp);
        String message = "this is a post created at " + timestamp + " by " + username;
        postManagement.createPost(username, message);
        return message;
    }

    @Test
    public void createPostMustReturnFalseIfUserDoesNotExist() {
        // Arrange
        // Act
        var result = postManagement.createPost("bleb", "some post");

        // Assert
        assertFalse(result);
    }

    @Test
    @DisplayName("createPost must return true")
    public void createPostMustReturnTrue() {
        // Arrange
        var albert = getAlbert();
        userManagement.createUser(albert);
        time.setCurrentTime(1000);
        String post = "this is a nice post, tweeet tweet";

        // Act
        var result = postManagement.createPost(albert.getUsername(), post);

        // Assert
        assertTrue(result);
    }

    @Test
    public void createdPostMustBeRetrieved() {
        // Arrange
        createAlbert();
        String createdMessage = createPost(albert.getUsername(), 100);

        // Act
        var posts = postManagement.getPosts(albert.getUsername());

        // Assert
        assertEquals(createdMessage, posts.get(0).getMessage());
        assertEquals(100, posts.get(0).getTimestamp());
    }

    @ParameterizedTest
    @CsvSource(value = {"150, 350", "200, 300"})
    public void createdPostMustBeRetrievedIfInsideInterval(String start, String end) {
        // Arrange
        createAlbert();
        String msg1 = createPost(albert.getUsername(), 100);
        String msg2 = createPost(albert.getUsername(), 200);
        String msg3 = createPost(albert.getUsername(), 300);
        String msg4 = createPost(albert.getUsername(), 400);

        // Act
        long startTime = Long.parseLong(start);
        long endTime = Long.parseLong(end);
        var posts = postManagement.getPostsBetween(albert.getUsername(), startTime, endTime);

        // Assert
        assertEquals(2, posts.size());
        assertEquals(msg2, posts.get(0).getMessage());
        assertEquals(msg3, posts.get(1).getMessage());
    }
}
