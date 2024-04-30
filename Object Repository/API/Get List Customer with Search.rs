<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get List Customer with Search</name>
   <tag></tag>
   <elementGuidId>6d69388a-60ed-42e6-9a0b-802523fed9d1</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${bearer_token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bearer_token}</value>
      <webElementGuid>96cd8812-5cdf-4641-b379-1c0efefe5eab</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://34.128.87.8:8081/customer?search=${search}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.bearerToken</defaultValue>
      <description></description>
      <id>14b3e359-3bf2-4f40-9c38-1c9fbc5194fa</id>
      <masked>false</masked>
      <name>bearer_token</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Prospect Customer').getValue(2, 1)</defaultValue>
      <description></description>
      <id>3a80edc5-7a6b-4116-8f0a-73447c5eac1a</id>
      <masked>false</masked>
      <name>search</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.search_keyword = WS.getElementPropertyValue(response, 'data[0].initial')

System.out.println(&quot;Search keyword = &quot; + GlobalVariable.search_keyword)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
