<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT_UpdatePost</name>
   <tag></tag>
   <elementGuidId>03144cf6-a54d-4e22-af73-4cdca73c93d4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;${title}\&quot;,\n  \&quot;body\&quot;: \&quot;${content}\&quot;,\n  \&quot;userId\&quot;: ${userId},\n  \&quot;id\&quot;: ${postId}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a2bcc8cf-61ae-432a-8fbc-27ae9edfea22</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/posts/${postId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>bc78c67d-4afb-4136-9181-a9f25e756b61</id>
      <masked>false</masked>
      <name>postId</name>
   </variables>
   <variables>
      <defaultValue>'foo'</defaultValue>
      <description></description>
      <id>22463ac4-130b-448d-8df5-7a9cdd67fb98</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'bar'</defaultValue>
      <description></description>
      <id>af5c7e6c-dbf1-47e7-a971-63328a2da17b</id>
      <masked>false</masked>
      <name>content</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>c32ecb6a-2e71-46b1-a906-2e75f255a00d</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
