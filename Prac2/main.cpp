//Richard Lastrucci u20430168
//Thabo Chesane u20507102
#include <stdio.h>
#include <string.h> 
#include <stdlib.h>
#include <errno.h>
#include <unistd.h> 
#include <arpa/inet.h> 
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <sys/time.h> 
#include <string>
#include <fstream>
#include <iostream>
#define PORT 8000
#define numClients 50
using namespace std;
//Global variables.

int addrlen;
int client_socket[numClients];
struct sockaddr_in address;
string client_buffer[numClients];

string convertToString(char* a, int size)
{
    int i;
    string s = "";
    for (i = 0; i < size; i++) {
        s = s + a[i];
    }
    return s;
}
 
string TEXT_RESET = "\\u001B[0m";
string TEXT_BLACK = "\\u001B[30m";
string TEXT_RED = "\\u001B[31m";
string TEXT_GREEN = "\\u001B[32m";
string TEXT_YELLOW = "\\u001B[33m";
string TEXT_BLUE = "\\u001B[34m";
string TEXT_PURPLE = "\\u001B[35m";
string TEXT_CYAN = "\\u001B[36m";
string TEXT_WHITE = "\\u001B[37m";
string CLEAR_SCREEN = "\\u001B[2J";

//messages 
void sendMessage(int clientSocket, string message){

    const void *a = message.c_str();
    send(clientSocket, a, message.length() + 1, 0);
}
void welcome(int clientSocket){
    string mystring = "\\u001B[2JWelcome to Richard's and Thabo's server.\r\nCommands Available:\r\nveiw - this shows the contents of the database.\r\nadd name,phone - this adds 'name' and their phone number to the database.\r\nremove name - this removes 'name' from the database.\r\nsearch name - this searches for 'name' and returns the data of 'name'.\r\nhelp - this shows this menu.\r\nquit - quits the server.\r\ninfo - for documentation\r\n";
    const void * a = mystring.c_str();
    send(clientSocket, a, mystring.length() + 1, 0);
}

void interprate(string mystring, int clientSocket, int index){
    if(mystring == "help"){
        welcome(clientSocket);
    }
    
    else if(mystring == "quit"){
        string output = TEXT_GREEN;
        output += "Good bye thank you for using the server!\r\n";
        output += TEXT_RESET;
        sendMessage(clientSocket,output);

        getpeername(clientSocket, (struct sockaddr*)&address , (socklen_t*)&addrlen);
        cout<<TEXT_YELLOW<<"Host disconnected IP: "<< inet_ntoa(address.sin_addr) <<" Port: "<< ntohs(address.sin_port)<<TEXT_RESET <<endl;
       
						
        close(clientSocket);
		client_socket[index] = 0;
        client_buffer[index] = "";
    }
    else if(mystring == "info"){
        //send information.
        string output;
        output += CLEAR_SCREEN;
        //documentation about the softawer as a whole.
        output += "Welcome to the documenation of the Taku and Thabo server.\r\n";
        output += "General information:\r\n";
        output += TEXT_RED;
        output += "Error message will be displayed in Red\r\n";
        output += TEXT_RESET;
        output += TEXT_GREEN;
        output += "Success queries will be displayed in Green\r\n";
        output += TEXT_RESET;
        output += TEXT_YELLOW;
        output += "Warning messages are displayed in Yellow\r\n";
        output += TEXT_RESET;

        output += "\r\n";
        //documentation about the add
        output += "add\r\n";
        output += "The add command adds data to the database in the following format.\r\n";
        output += "Name Surname number (note we use space as the deliminator)\r\n";
        output += "\r\n";            
        //documentation about the delete
        output += "delete\r\n";
        output += "The delete commands deletes data from the database, you need to provide the following:\r\n";
        output += "The Name Surname and Phone number of the user whose data you want to delete.\r\n";
        output += "\r\n";
        //documnetation about the search.
        output += "search\r\n";
        output += "The search command Find any entries in the database that contain your search criteria\r\n";
        output += "Example: search 081 - this will return all the fleids that conatin 081 in phone number\r\n";
        output += "The screen is cleared after this command\r\n";
        output += "\r\n";   
        //documentation about the view
        output += "view\r\n";
        output += "The view command shows all the contents of the database.\r\n";
        output += "The screen is cleared after this command\r\n";
        output += "\r\n";

        sendMessage(clientSocket,output);
    }
    //database operations
    else if(mystring == "view"){
        ifstream file("database.txt");
        if(file.peek() == EOF){ //empty file
            string output = TEXT_YELLOW;
            output += CLEAR_SCREEN;
            output += "The database is empty \r\n\\u001B[1B";
            output += TEXT_RESET;
            sendMessage(clientSocket,output);
        }
        else{
            //read the data
            string output = TEXT_GREEN;
            output += CLEAR_SCREEN;
            //also clear the screeen
            output += "The database contents are: \r\n\\u001B[1B";
            output += TEXT_RESET;
            sendMessage(clientSocket,output);
            string user;
            while (getline (file, user)) {
                user += "\r\n";
                sendMessage(clientSocket,user);
            }
            sendMessage(clientSocket,"\\u001B[1B");
        }
        file.close();
    }

    else if(mystring.find("add") != string::npos){
        //code for adding.  
        size_t pos = mystring.find("add");
        mystring.erase(pos, 4);
         string output ="";
        if(mystring.length() == 0){
            output = TEXT_RED;
            output += "You did not specify the users Name, Surname and Phone number of the user you wish to add.\r\n\\u001B[1B";
            output += TEXT_RESET;
            sendMessage(clientSocket,output);
        }
        else{
            fstream file("database.txt");
            string line;
            bool found = false;
            while(getline(file,line)){
                if(line == mystring){
                    found = true;
                     output = TEXT_RED;
                    output += mystring + " error the user already exists in the database.\r\n\\u001B[1B";
                    output += TEXT_RESET;
                    sendMessage(clientSocket,output);
        
                    file.close();
                    return;
                }
            }
            
            file.close();
            ofstream myfile("database.txt",ios_base::app);
            myfile << mystring + "\n";

            myfile.close();
            output = TEXT_GREEN;
            output += mystring + " has been succeffully added to the database\r\n\\u001B[1B";
            output += TEXT_RESET;
            sendMessage(clientSocket,output);
              
    
               
            
        }

    }
    else if(mystring.find("delete") != string::npos){
        //code for deleting
        size_t pos = mystring.find("delete");
        mystring.erase(pos, 7);
         string output ="";
        if(mystring.length() == 0){
            output = TEXT_RED;
            output += "You did not specify the Name, Surname and Phone number of the user you wish to delete.\r\n\\u001B[1B";
            output += TEXT_RESET;
            sendMessage(clientSocket,output);
        }
        else{
            fstream file("database.txt");
            //search for the user
            string temp;
            string line;
            bool found = false;
            while(getline(file,line)){
                if(line != mystring){
                    temp += line + "\n";
                }
                else{
                    found = true;
                }
            }
            if(found){
                file.close();
                file.open("database.txt",std::ofstream::out | std::ofstream::trunc);
                file << temp;     
                output = TEXT_GREEN;
                output += mystring + " has been succeffully deleted from the database.\r\n\\u001B[1B";
                output += TEXT_RESET;
                sendMessage(clientSocket,output);
            }
            else{
                output = TEXT_RED;
                output += mystring + " was not found in the database.\r\n\\u001B[1B";
                output += TEXT_RESET;
                sendMessage(clientSocket,output);
            }
            
            file.close();
            
        }
    }
    else if(mystring.find("search") != string::npos){
        //code for searching
        size_t pos = mystring.find("search");
        mystring.erase(pos, 7);
        string output ="";
        if(mystring.length() == 0){
            output = CLEAR_SCREEN;
            output += TEXT_RED;
            output += "Please specify and search input.\r\n\\u001B[1B";
            output += TEXT_RESET;
            sendMessage(clientSocket,output);
        }
        else{
        fstream file("database.txt");
        string line;
        bool found = false;
        while(getline(file,line)){
            if(line.find(mystring) != string::npos){
                if(!found){
                    output = CLEAR_SCREEN;
                    output += TEXT_GREEN;
                    output += "here are usernames in the database that contain: "+mystring+"\r\n\\u001B[1B";
                    output += TEXT_RESET;
                    sendMessage(clientSocket,output);
                }
                found = true;
                sendMessage(clientSocket, line + "\r\n");

            }
        }
        if(!found){
            output = CLEAR_SCREEN;
            output += TEXT_YELLOW;
            output += "No results that match your query were found.\r\n\\u001B[1B";
            output += TEXT_RESET;
            sendMessage(clientSocket,output);

        }
        file.close();
        }
        
    }
    else{
        string output = TEXT_RED;
        output += "Your Query is invalid type 'help' for information about the availabe commands\r\n\\u001B[1B";
        output += TEXT_RESET;
        sendMessage(clientSocket,output);
        
    }

}

int main(int argc , char *argv[])
{
    int master_socket; 
    char buffer[1024]; 
	fd_set readFDs;

	for (int i = 0; i < numClients; i++)
	{
		client_socket[i] = 0;
        client_buffer[i] = "";
    }	
	if((master_socket = socket(AF_INET , SOCK_STREAM , 0)) == 0)
	{
        cout<<"faile to create a master socket"<<endl;
		exit(EXIT_FAILURE);
	}
    int opt = 1;
    if( setsockopt(master_socket, SOL_SOCKET, SO_REUSEADDR, (char *)&opt, sizeof(opt)) < 0 )  
    {   
        cout<<"Could not setsockopt"<<endl;
        exit(EXIT_FAILURE);  
    }  
	//type of socket created
	address.sin_family = AF_INET;
	address.sin_addr.s_addr = INADDR_ANY;
	address.sin_port = htons( PORT );
		
	if (bind(master_socket, (struct sockaddr *)&address, sizeof(address))<0)
	{
		perror("bind failed");
		exit(EXIT_FAILURE);
    }
    cout<<"Listerning on port "<<PORT <<endl;
	if (listen(master_socket, 3) < 0)
	{
		perror("could not esstable a master socket");
		exit(EXIT_FAILURE);
	}
	addrlen = sizeof(address);
    cout<<"Wating for connections ..."<<endl;
	while(true)
	{
		FD_ZERO(&readFDs);
        int activity;
        int socketDecriptor;
        int new_socket;
		FD_SET(master_socket, &readFDs);
		int max_sd = master_socket;
			
		for(int i = 0; i < numClients ; i++)
		{
			socketDecriptor = client_socket[i];
			if(socketDecriptor > 0)
            {
                FD_SET( socketDecriptor , &readFDs);
            }
			if(socketDecriptor > max_sd)
            {
                max_sd = socketDecriptor;
            }
		}
		activity = select(max_sd + 1,&readFDs,NULL,NULL,NULL);
		if((activity < 0) && (errno!=EINTR))
		{
            cout<<"select error"<<endl;
		}
			
		if(FD_ISSET(master_socket, &readFDs))
		{
			if((new_socket = accept(master_socket,(struct sockaddr *)&address, (socklen_t*)&addrlen))<0)
			{
				perror("accept");
				exit(EXIT_FAILURE);
			}
            cout<<TEXT_GREEN<<"New connection, the socket fd is: "<< new_socket<<" the IP is: "<<inet_ntoa(address.sin_addr)<<" the port is: "<< ntohs(address.sin_port)<<TEXT_RESET<<endl;
			//send new connection greeting message
            string m = "\\u001B[2JWelcome to Taku and Thabo server.\r\nCommands Available:\r\nveiw - this shows the contents of the database.\r\nadd name,phone - this adds 'name' and their phone number to the database.\r\nremove name - this removes 'name' from the database.\r\nsearch name - this searches for 'name' and returns the data of 'name'.\r\nhelp - this shows this menu.\r\nquit - quits the server.\r\ninfo - for documentation\r\n";
            const char *message = m.c_str(); 
			if(send(new_socket, message, strlen(message), 0) != strlen(message) )
			{
				cout<<"could not send a message to the client"<<endl;
			}
            else
            {
                cout<<"Welcome message sent succesffully to : "<<inet_ntoa(address.sin_addr)<<endl;
            }
			for (int i = 0; i < numClients; i++)
			{
				if( client_socket[i] == 0 )
				{
					client_socket[i] = new_socket;
                    cout<<"Adding to list of sockets as "<< i<<endl;
					break;
				}
			}
		}
		for (int i = 0; i < numClients; i++)
		{
			socketDecriptor = client_socket[i];
			if(FD_ISSET( socketDecriptor , &readFDs))
			{
                int val;
				if ((val = read( socketDecriptor , buffer, 1024)) == 0)
				{
					//client disconnected.
					getpeername(socketDecriptor , (struct sockaddr*)&address,(socklen_t*)&addrlen);
                    cout<<TEXT_YELLOW<<"Host disconnected IP: "<< inet_ntoa(address.sin_addr) <<" Port: "<< ntohs(address.sin_port)<<TEXT_RESET<<endl;
					close( socketDecriptor );
					client_socket[i] = 0;
                    client_buffer[i] = "";
				}
                else
				{
					buffer[val] = '\0';
                    string test = "";
                    bool processing = false;
                    for(int k=0; k<val; k++){
                        if(buffer[k] == '\n'){
                            processing = true;
                            test += "\\u001B[2J\\u001B[1A";
                            //interprate
                            cout<<"Trying to interprate: "<<client_buffer[i]<<endl;
                            interprate(client_buffer[i],socketDecriptor,i);
                            client_buffer[i] = "";
                        }
                    }
                    if(!processing){
                        string character = convertToString(buffer,val);
                        client_buffer[i] += character;
                        test += "\\u001B[1B" +character+ "\\u001B[1A\\u001B[1D";
                        const void * mystring = test.c_str();
                        send(socketDecriptor ,  mystring , test.length() , 0 );
                    }
				}
			}
		}
	}
	return 0;
}
