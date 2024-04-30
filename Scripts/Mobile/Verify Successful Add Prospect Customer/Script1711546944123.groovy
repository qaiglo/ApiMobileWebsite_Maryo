import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Mobile/Verify Successful Login'), [:], FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Mobile/Menu Customer Update/menu_customerUpdate'), GlobalVariable.timeout)

Mobile.tap(findTestObject('Mobile/common/btn_add'), GlobalVariable.timeout)

Mobile.tap(findTestObject('Mobile/Menu Customer Update/btn_prospectCustomer'), GlobalVariable.timeout)

Mobile.delay(GlobalVariable.timeout)

Mobile.takeScreenshotAsCheckpoint('Before Add Prospect Customer', [])

Mobile.tap(findTestObject('Mobile/Menu Customer Update/txt_customerName'), GlobalVariable.timeout)

Mobile.setText(findTestObject('Mobile/Menu Customer Update/txt_customerName'), findTestData('Prospect Customer').getValue(
        1, 1), GlobalVariable.timeout)

Mobile.tap(findTestObject('Mobile/Menu Customer Update/txt_customerInitial'), GlobalVariable.timeout)

Mobile.setText(findTestObject('Mobile/Menu Customer Update/txt_customerInitial'), findTestData('Prospect Customer').getValue(
        2, 1), GlobalVariable.timeout)

Mobile.tap(findTestObject('Mobile/Menu Customer Update/dropdown_classification'), GlobalVariable.timeout)

Mobile.tap(findTestObject('Mobile/Menu Customer Update/txt_classification_commercial'), GlobalVariable.timeout)

Mobile.delay(GlobalVariable.timeout, FailureHandling.STOP_ON_FAILURE)

Mobile.tap(findTestObject('Mobile/common/btn_create'), GlobalVariable.timeout)

Mobile.verifyElementExist(findTestObject('Mobile/common/msg_dataSaveSuccess'), GlobalVariable.timeout)

Mobile.delay(2)

Mobile.takeScreenshotAsCheckpoint('After Add Prospect Customer', [])

