package com.example.myedmaproject.generated.valuedomains;

import com.example.myedmaproject.generated.edma_ffs;
import com.example.myedmaproject.generated.valuedomains.external.EDMA_ExternalConstraints;
import com.example.myedmaproject.generated.valuedomains.impl.MyValueDomainImpl;
import java.io.DataInput;
import java.io.DataOutput;
import java.io.IOException;
import org.abstractica.edma.valuedomains.IMetaValueDomain;
import org.abstractica.edma.valuedomains.exceptions.InvalidValueException;
import org.abstractica.edma.valuedomains.userinput.ITerminal;
import org.abstractica.edma.valuedomains.userinput.ValueDomainInput;

/**
 * The representation of a value from the value domain: MyValueDomain
 */
public abstract class MyValueDomain implements Comparable<MyValueDomain>
{
    protected static final IMetaValueDomain edma_domain = edma_ffs.environment.getValueDomainDefinitions().getValueDomain(0);



    /**
     * Get a value from a terminal
     * @param terminal  The terminal to get the value from
     * @return          The MyValueDomain from the terminal
     */
    public static MyValueDomain fromTerminal(ITerminal terminal)
    {
        ValueDomainInput vdi = new ValueDomainInput(terminal, EDMA_ExternalConstraints.instance);
        return new MyValueDomainImpl(vdi.getValue(edma_domain));
    }

    /**
     * Get a value from its string representation
     * @param s  The String to parse
     * @return   The MyValueDomain from the string representation
     */
    public static MyValueDomain fromString(String s) throws InvalidValueException
    {
        Object res = edma_domain.valueFromString(s, EDMA_ExternalConstraints.instance);
        return new MyValueDomainImpl(res);
    }

    /**
     * Reads and validates a value from a stream
     * @param in  A data input interface for the stream to read from
     * @return    The MyValueDomain read from the stream
     */
    public static MyValueDomain fromStream(DataInput in) throws IOException, InvalidValueException
    {
        Object res = edma_domain.readValue(in, EDMA_ExternalConstraints.instance);
        return new MyValueDomainImpl(res);
    }

    /**
     * Reads a value from a stream without validating the value
     * @param in  A data input interface for the stream to read from
     * @return    The MyValueDomain read from the stream
     */
    public static MyValueDomain fromStreamNoValidate(DataInput in) throws IOException
    {
        Object res = edma_domain.readValueNoValidate(in);
        return new MyValueDomainImpl(res);
    }

    /**
     * Call this method to create a new MyValueDomain value. 
     * @param value  The value of the MyValueDomain to be created.
     * @return       The newly created MyValueDomain value
     */
    public static MyValueDomain create(Integer value) throws InvalidValueException
    {
        MyValueDomainImpl.edma_validate(value);
        return new MyValueDomainImpl(MyValueDomainImpl.edma_create(value));
    }

    /**
     * Call this method to test if the provided Integer is a valid MyValueDomain
     * @param value  The Integer value to be tested
     * @return       true if the provided Integer is a valid MyValueDomain
     */
    public static boolean isValidMyValueDomain(Integer value)
    {
        try
        {
            MyValueDomainImpl.edma_validate(value);
        }
        catch(InvalidValueException e)
        {
            return false;
        }
        return true;
    }



    /**
     * Writes this value to a stream
     * @param out  Interface to data output stream
     */
    public abstract void toStream(DataOutput out) throws IOException;

    /**
     * returns the Integer value that is stored in this MyValueDomain
     * @return  The Integer value stored in this MyValueDomain
     */
    public abstract Integer value();
}
