package dk.cphbusiness.mrv.twitterclone;

import dk.cphbusiness.mrv.twitterclone.contract.PostManagement;
import dk.cphbusiness.mrv.twitterclone.contract.UserManagement;
import dk.cphbusiness.mrv.twitterclone.dto.UserCreation;
import dk.cphbusiness.mrv.twitterclone.impl.PostManagementImpl;
import dk.cphbusiness.mrv.twitterclone.impl.UserManagementImpl;
import org.junit.jupiter.api.*;
import redis.clients.jedis.Jedis;
import redis.clients.jedis.exceptions.JedisConnectionException;
import redis.clients.jedis.exceptions.JedisException;
import redis.clients.jedis.JedisPool;
import redis.clients.jedis.JedisPoolConfig;

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
public class TestBase {

    // Don't use "um" and "pm". Code is for people, people.
    // Nobody is going to understand anything if you just write "um" and "pm"
    // as variable names.
    protected UserManagement userManagement;
    protected PostManagement postManagement;
    protected TimeFake time;
    private Jedis jedis;

    private String host = "localhost";
    private int port = 22228;

    @BeforeAll
    public void setup() {

        /*
         * The following line starts a new container with redis, and runs integration
         * tests on it.
         * 
         * Depending on your setup, it might not work. If it doesn't work, try this in
         * your terminal: docker pull testcontainers/ryuk:0.3.0
         * 
         * If it still doesn't work, you can comment out the line, BUT then the
         * integration tests will be run against your local redis, AND IT WILL FLUSH DB
         * 9! To prove that you have read this warning, delete the exception below.
         */
        

        jedis = new Jedis(host, port);
        jedis.select(0);
        time = new TimeFake();

        userManagement = new UserManagementImpl(jedis);
        postManagement = new PostManagementImpl(jedis);
    }

    @BeforeEach
    public void beforeEach() {
        jedis.flushDB();
    }

    protected UserCreation getAlbert() {
        return new UserCreation("ahr", "Albert", "RuserManagementle", "aou87", "1951-03-03");
    }

    protected UserCreation getBenny() {
        return new UserCreation("ben", "Benny", "Juuel", "htiuh9", "1941-04-04");
    }

    protected UserCreation getCarl() {
        return new UserCreation("crl", "Carl", "Vladimirovich", "tytytt", "1981-05-05");
    }

    protected UserCreation getDennis() {
        return new UserCreation("den", "Dennis", "Olsen", "uhcah834", "1982-06-06");
    }
}
