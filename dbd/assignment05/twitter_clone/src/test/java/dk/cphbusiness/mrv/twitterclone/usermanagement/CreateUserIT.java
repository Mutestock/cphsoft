package dk.cphbusiness.mrv.twitterclone.usermanagement;

import dk.cphbusiness.mrv.twitterclone.TestBase;
import dk.cphbusiness.mrv.twitterclone.dto.UserCreation;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

public class CreateUserIT extends TestBase {
    @Test
    public void createUserMustReturnFalseIfUserExists() {
        // Arrange
        UserCreation userCreator = getAlbert();
        boolean res = userManagement.createUser(userCreator);
        assertTrue(res);

        // Act
        res = userManagement.createUser(userCreator);

        // Assert
        assertFalse(res);
    }

    @Test
    public void userMustExistWhenCreated() {
        // Arrange
        var user = getAlbert();
        userManagement.createUser(user);

        // Act
        var userOverview = userManagement.getUserOverview(user.getUsername());

        // Assert
        assertEquals(user.getFirstname(), userOverview.getFirstname());
        assertEquals(user.getLastname(), userOverview.getLastname());
    }

}
